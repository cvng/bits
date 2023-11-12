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
use bits_data::bid;
use bits_data::sea_orm::EntityTrait;
use bits_data::Amount;
use bits_data::AuctionId;
use bits_data::Bid;
use bits_data::BidCreated;
use bits_data::BidId;
use bits_data::Event;
use bits_data::PersonId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidInput {
  pub auction_id: AuctionId,
  pub buyer_id: PersonId,
  pub amount: Amount,
}

impl BidInput {
  pub fn type_name() -> &'static str {
    "BidInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("buyerId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("amount", TypeRef::named_nn(TypeRef::INT)))
  }
}

#[derive(Clone, Serialize)]
pub struct BidResult {
  pub bid: Bid,
}

impl BidResult {
  pub fn type_name() -> &'static str {
    "BidResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "bid",
      TypeRef::named_nn("Bid"),
      |ctx| {
        FieldFuture::new(async move {
          Ok(Some(FieldValue::owned_any(
            ctx
              .parent_value
              .try_downcast_ref::<Self>()
              .cloned()
              .unwrap()
              .bid,
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
  #[error("bid {0:?} not found")]
  NotFound(BidId),
}

pub struct BidCommand<'a> {
  client: &'a Client,
}

impl<'a> Command for BidCommand<'a> {
  type Error = Error;
  type Input = BidInput;
  type Result = BidResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    Ok(vec![Event::BidCreated {
      data: BidCreated {
        id: BidId::new_v4(),
        auction_id: input.auction_id,
        buyer_id: input.buyer_id,
        amount: input.amount,
      },
    }])
  }

  async fn apply(
    &self,
    _input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let bid_id = events
      .iter()
      .find_map(|event| match event {
        Event::BidCreated { data, .. } => Some(data.id),
        _ => None,
      })
      .unwrap();

    let bid = bid::Entity::find_by_id(bid_id)
      .one(&self.client.connection)
      .await
      .map_err(DispatchError::Database)?
      .ok_or(Error::NotFound(bid_id))?;

    Ok(Self::Result { bid })
  }
}

pub async fn bid(client: &Client, input: BidInput) -> Result<BidResult, Error> {
  BidCommand { client }.run(input).await
}
