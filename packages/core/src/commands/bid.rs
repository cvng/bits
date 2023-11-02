use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Amount;
use bits_data::AuctionId;
use bits_data::Bid;
use bits_data::BidId;
use bits_data::Event;
use bits_data::UserId;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidInput {
  pub auction_id: AuctionId,
  pub bidder_id: UserId,
  pub amount: Amount,
}

impl BidInput {
  pub fn type_name() -> &'static str {
    "BidInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("bidderId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("amount", TypeRef::named_nn(TypeRef::INT)))
  }
}

#[derive(Serialize)]
pub struct BidResult {
  pub bid: Bid,
}

impl BidResult {
  pub fn type_name() -> &'static str {
    "BidResult"
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
  #[error("bid not created")]
  NotCreated,
}

#[derive(Default)]
pub struct BidCommand {}

impl Command for BidCommand {
  type Error = Error;
  type Event = Event;
  type Input = BidInput;
  type Result = BidResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::bid_created(
      BidId::new_v4(),
      input.auction_id,
      input.bidder_id,
      input.amount,
    )])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::BidCreated { data, .. } => Some(BidResult {
        bid: Bid {
          id: data.id,
          created: None,
          updated: None,
          auction_id: data.auction_id,
          bidder_id: data.bidder_id,
          concurrent_amount: None,
          amount: data.amount,
        },
      }),
      _ => None,
    })
  }
}

pub async fn bid(client: &Client, input: BidInput) -> Result<BidResult, Error> {
  dispatcher::dispatch(client, BidCommand {}.handle(input)?)
    .await
    .map(BidCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_bid() {
  let now = "2023-10-17T03:16:49.225067Z"
    .parse::<bits_data::DateTime>()
    .unwrap();

  let auction = bits_data::Auction {
    id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
    created: None,
    updated: None,
    show_id: "28e9d842-0918-460f-9cd9-7245dbba1966".parse().unwrap(),
    product_id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
    started: Some("2023-10-16T23:56:27.365540Z".parse().unwrap()),
    expired: Some(
      now + bits_data::Duration::seconds(bits_data::AUCTION_TIMEOUT_SECS),
    ),
  };

  let input = BidInput {
    auction_id: auction.id,
    bidder_id: "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68".parse().unwrap(),
    amount: 100.into(),
  };

  let events = BidCommand {}.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "BidCreated",
      "data": {
        "id": "d971d1f2-986b-4883-bbcc-e318c2060022",
        "auction_id": "f7223b3f-4045-4ef2-a8c3-058e1f742f2e",
        "bidder_id": "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68",
        "amount": "100"
      }
    }
  ]
  "###);
}
