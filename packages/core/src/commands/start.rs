use super::Command;
use crate::dispatcher;
use crate::dispatcher::DispatchError;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::auction;
use bits_data::sea_orm::DbErr;
use bits_data::sea_orm::EntityTrait;
use bits_data::Auction;
use bits_data::AuctionStarted;
use bits_data::Event;
use bits_data::ShowId;
use bits_data::ShowStarted;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartInput {
  pub auction_id: ShowId,
}

impl StartInput {
  pub fn type_name() -> &'static str {
    "StartInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
  }
}

#[derive(Clone, Serialize)]
pub struct StartResult {
  pub auction: Auction,
}

impl StartResult {
  pub fn type_name() -> &'static str {
    "StartResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "auction",
      TypeRef::named_nn("Show"),
      |ctx| {
        FieldFuture::new(async move {
          Ok(Some(FieldValue::owned_any(
            ctx
              .parent_value
              .try_downcast_ref::<Self>()
              .cloned()
              .unwrap()
              .auction,
          )))
        })
      },
    ))
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("internal: db error")]
  Db(#[from] DbErr),
  #[error("internal: dispatch error")]
  Dispatch(#[from] DispatchError),
  #[error("show not created")]
  NotCreated,
  #[error("show not found")]
  NotFound,
}

pub struct StartCommand {
  auction: Auction,
}

impl Command for StartCommand {
  type Error = Error;
  type Event = Event;
  type Input = StartInput;
  type Result = StartResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![
      Event::ShowStarted {
        data: ShowStarted {
          id: self.auction.show_id,
        },
      },
      Event::AuctionStarted {
        data: AuctionStarted {
          id: input.auction_id,
          auction: self.auction.clone(),
        },
      },
    ])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::AuctionStarted { data, .. } => Some(StartResult {
        auction: data.auction.clone(),
      }),
      _ => None,
    })
  }
}

pub async fn start(
  client: &Client,
  input: StartInput,
) -> Result<StartResult, Error> {
  let auction = auction::Entity::find_by_id(input.auction_id)
    .one(&client.connection)
    .await?
    .ok_or(Error::NotFound)?;

  let events = StartCommand { auction }.handle(input)?;

  dispatcher::dispatch(client, events)
    .await
    .map(StartCommand::apply)
    .map_err(Error::Dispatch)?
    .ok_or(Error::NotCreated)
}
