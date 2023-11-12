use crate::command::Command;
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
use bits_data::sea_orm::EntityTrait;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionStarted;
use bits_data::Event;
use bits_data::ShowStarted;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartInput {
  pub auction_id: AuctionId,
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
      TypeRef::named_nn("Auction"),
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
  Dx(#[from] DispatchError),
  #[error("auction {0:?} not found")]
  NotFound(AuctionId),
}

pub struct StartCommand<'a> {
  client: &'a Client,
}

impl<'a> Command for StartCommand<'a> {
  type Error = Error;
  type Input = StartInput;
  type Result = StartResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    let auction = auction::Entity::find_by_id(input.auction_id)
      .one(&self.client.connection)
      .await
      .map_err(DispatchError::Database)?
      .ok_or(Error::NotFound(input.auction_id))?;

    Ok(vec![
      Event::ShowStarted {
        data: ShowStarted {
          id: auction.show_id,
        },
      },
      Event::AuctionStarted {
        data: AuctionStarted {
          id: input.auction_id,
        },
      },
    ])
  }

  async fn apply(
    &self,
    input: Self::Input,
    _events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let auction = auction::Entity::find_by_id(input.auction_id)
      .one(&self.client.connection)
      .await
      .map_err(DispatchError::Database)?
      .ok_or(Error::NotFound(input.auction_id))?;

    Ok(Self::Result { auction })
  }
}

pub async fn start(
  client: &Client,
  input: StartInput,
) -> Result<StartResult, Error> {
  StartCommand { client }.run(input).await
}
