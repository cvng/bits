use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::Event;
use bits_data::ProductId;
use bits_data::ShowId;
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

#[derive(Serialize)]
pub struct CreateAuctionResult {
  pub auction: Auction,
}

impl CreateAuctionResult {
  pub fn type_name() -> &'static str {
    "CreateAuctionResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "id".to_string(),
      TypeRef::named_nn(TypeRef::ID),
      |ctx| {
        FieldFuture::new(
          async move { Ok(ctx.parent_value.as_value().cloned()) },
        )
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
    Ok(vec![Event::auction_created(
      AuctionId::new_v4(),
      input.show_id,
      input.product_id,
    )])
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
  dispatcher::dispatch(client, CreateAuctionCommand {}.handle(input)?)
    .await
    .map(CreateAuctionCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_create_auction() {
  let show = bits_data::Show {
    id: "048b47f4-3010-43ae-84c1-8088ab8488a8".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: bits_data::UserId::new_v4(),
    name: "name".parse().unwrap(),
    started: None,
  };

  let product = bits_data::Product {
    id: "2b1af787-2d94-4224-a2fc-1d8d155537c0".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: bits_data::UserId::new_v4(),
    name: "name".parse().unwrap(),
  };

  let input = CreateAuctionInput {
    show_id: show.id,
    product_id: product.id,
  };

  let events = CreateAuctionCommand {}.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "AuctionCreated",
      "data": {
        "id": "0047390f-06fe-4f14-857d-048e89225fe0",
        "show_id": "048b47f4-3010-43ae-84c1-8088ab8488a8",
        "product_id": "2b1af787-2d94-4224-a2fc-1d8d155537c0"
      }
    }
  ]
  "###);
}
