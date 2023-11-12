use crate::command::Command;
use crate::dispatcher::InternalError;
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
use bits_data::AuctionCreated;
use bits_data::AuctionId;
use bits_data::Event;
use bits_data::ProductId;
use bits_data::ShowId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuctionInput {
  pub show_id: ShowId,
  pub product_id: ProductId,
}

impl CreateAuctionInput {
  pub fn type_name() -> &'static str {
    "CreateAuctionInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("showId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("productId", TypeRef::named_nn(TypeRef::ID)))
  }
}

#[derive(Clone, Serialize)]
pub struct CreateAuctionResult {
  pub auction: Auction,
}

impl CreateAuctionResult {
  pub fn type_name() -> &'static str {
    "CreateAuctionResult"
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
  Internal(#[from] InternalError),
  #[error("auction {0:?} not found")]
  NotFound(AuctionId),
}

pub struct CreateAuctionCommand<'a> {
  client: &'a Client,
}

impl<'a> Command for CreateAuctionCommand<'a> {
  type Error = Error;
  type Input = CreateAuctionInput;
  type Result = CreateAuctionResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    Ok(vec![Event::AuctionCreated {
      data: AuctionCreated {
        id: AuctionId::new_v4(),
        show_id: input.show_id,
        product_id: input.product_id,
      },
    }])
  }

  async fn apply(
    &self,
    _input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let auction_id = events
      .iter()
      .find_map(|event| match event {
        Event::AuctionCreated { data, .. } => Some(data.id),
        _ => None,
      })
      .unwrap();

    let auction = auction::Entity::find_by_id(auction_id)
      .one(&self.client.connection)
      .await
      .map_err(InternalError::Database)?
      .ok_or(Error::NotFound(auction_id))?;

    Ok(Self::Result { auction })
  }
}

pub async fn create_auction(
  client: &Client,
  input: CreateAuctionInput,
) -> Result<CreateAuctionResult, Error> {
  CreateAuctionCommand { client }.run(input).await
}
