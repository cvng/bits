use super::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Auction;
use bits_data::AuctionCreated;
use bits_data::AuctionId;
use bits_data::Event;
use bits_data::ProductId;
use bits_data::ShowId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Deserialize)]
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
  #[error("auction not created")]
  NotCreated,
}

pub struct CreateAuctionCommand {}

impl Command for CreateAuctionCommand {
  type Error = Error;
  type Event = Event;
  type Input = CreateAuctionInput;
  type Result = CreateAuctionResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::AuctionCreated {
      data: AuctionCreated {
        id: AuctionId::new_v4(),
        show_id: input.show_id,
        product_id: input.product_id,
      },
    }])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::AuctionCreated { data, .. } => Some(CreateAuctionResult {
        auction: Auction {
          id: data.id,
          created: None,
          updated: None,
          show_id: data.show_id,
          product_id: data.product_id,
          started: None,
          expired: None,
        },
      }),
      _ => None,
    })
  }
}

pub async fn create_auction(
  client: &Client,
  input: CreateAuctionInput,
) -> Result<CreateAuctionResult, Error> {
  let events = CreateAuctionCommand {}.handle(input)?;

  dispatcher::dispatch(client, events)
    .await
    .map(CreateAuctionCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_create_auction() {
  let input = CreateAuctionInput {
    show_id: "048b47f4-3010-43ae-84c1-8088ab8488a8".parse().unwrap(),
    product_id: "2b1af787-2d94-4224-a2fc-1d8d155537c0".parse().unwrap(),
  };

  let events = CreateAuctionCommand {}.handle(input).unwrap();

  insta::assert_json_snapshot!(events, { "[0].data.id" => "[uuid]" },  @r###"
  [
    {
      "type": "auction_created",
      "data": {
        "id": "[uuid]",
        "show_id": "048b47f4-3010-43ae-84c1-8088ab8488a8",
        "product_id": "2b1af787-2d94-4224-a2fc-1d8d155537c0"
      }
    }
  ]
  "###);
}
