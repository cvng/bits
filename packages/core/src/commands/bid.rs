use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::dynamic::indexmap::IndexMap;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_graphql::Name;
use async_graphql::Value;
use bits_data::Amount;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::Bid;
use bits_data::BidId;
use bits_data::Duration;
use bits_data::Event;
use bits_data::ProductId;
use bits_data::UserId;
use bits_data::Utc;
use bits_data::AUCTION_REFRESH_SECS;
use thiserror::Error;

pub struct BidInput {
  pub auction_id: AuctionId,
  pub bidder_id: UserId,
  pub amount: Amount,
}

impl BidInput {
  pub fn to_input_object() -> InputObject {
    InputObject::new("BidInput")
      .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("bidderId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("amount", TypeRef::named_nn(TypeRef::INT)))
  }
}

pub struct BidResult {
  pub bid: Bid,
}

impl BidResult {
  pub fn to_object() -> Object {
    Object::new("BidResult").field(Field::new(
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

impl From<BidResult> for Value {
  fn from(value: BidResult) -> Self {
    let mut map = IndexMap::new();
    map.insert(Name::new("id"), value.bid.id.to_string().into());
    Value::Object(map)
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("auction not found: {0}")]
  AuctionNotFound(AuctionId),
  #[error("auction not ready: {0}")]
  AuctionNotReady(AuctionId),
  #[error("auction not started: {0}")]
  AuctionNotStarted(AuctionId),
  #[error("auction expired: {0}")]
  AuctionExpired(AuctionId),
  #[error("invalid bid amount: {0}")]
  InvalidAmount(Amount),
  #[error("bid not created")]
  NotCreated,
  #[error("product not found: {0}")]
  ProductNotFound(ProductId),
}

#[derive(Default)]
pub struct BidCommand {
  pub auction: Option<Auction>,
  pub bid: Option<Bid>,
}

impl Command for BidCommand {
  type Error = Error;
  type Event = Event;
  type Input = BidInput;
  type Result = BidResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let mut auction = self
      .auction
      .ok_or(Error::AuctionNotFound(input.auction_id))?;

    let bid = self.bid.ok_or(Error::NotCreated)?;

    auction
      .started
      .ok_or(Error::AuctionNotStarted(auction.id))?;

    let expired_at = auction
      .expired
      .ok_or(Error::AuctionNotStarted(auction.id))?;

    (bid.created_at < expired_at)
      .then_some(())
      .ok_or(Error::AuctionExpired(auction.id))?;

    auction.expired =
      Some(bid.created_at + Duration::seconds(AUCTION_REFRESH_SECS));

    Ok(vec![
      Event::bid_created(bid),
      Event::auction_revived(auction),
    ])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::BidCreated { payload } => Some(BidResult { bid: payload.bid }),
      _ => None,
    })
  }
}

pub async fn bid(input: BidInput) -> Result<BidResult, Error> {
  let auction = database::db().auctions.get(&input.auction_id).cloned();

  let bid = Some(Bid {
    id: BidId::new(),
    auction_id: input.auction_id,
    bidder_id: input.bidder_id,
    amount: input.amount,
    created_at: Utc::now(),
  });

  BidCommand { auction, bid }
    .handle(input)
    .map(dispatcher::dispatch)?
    .map(BidCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_bid() {
  let now = "2023-10-17T03:16:49.225067Z"
    .parse::<bits_data::DateTime>()
    .unwrap();

  let auction = Some(Auction {
    id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
    show_id: "28e9d842-0918-460f-9cd9-7245dbba1966".parse().unwrap(),
    product_id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
    started: Some("2023-10-16T23:56:27.365540Z".parse().unwrap()),
    expired: Some(now + Duration::seconds(bits_data::AUCTION_TIMEOUT_SECS)),
  });

  let input = BidInput {
    auction_id: auction.as_ref().unwrap().id,
    bidder_id: "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68".parse().unwrap(),
    amount: 100,
  };

  let bid = Some(Bid {
    id: "bcd0ab01-96f0-4469-a3e6-254afe70b74f".parse().unwrap(),
    auction_id: input.auction_id,
    bidder_id: input.bidder_id,
    amount: input.amount,
    created_at: "2023-10-16T04:41:02.676340Z".parse().unwrap(),
  });

  let events = BidCommand { auction, bid }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "bid_created",
      "payload": {
        "bid": {
          "id": "bcd0ab01-96f0-4469-a3e6-254afe70b74f",
          "auction_id": "f7223b3f-4045-4ef2-a8c3-058e1f742f2e",
          "bidder_id": "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68",
          "amount": 100,
          "created_at": "2023-10-16T04:41:02.676340Z"
        }
      }
    },
    {
      "type": "auction_revived",
      "payload": {
        "auction": {
          "id": "f7223b3f-4045-4ef2-a8c3-058e1f742f2e",
          "show_id": "28e9d842-0918-460f-9cd9-7245dbba1966",
          "product_id": "6bc8e88e-fc47-41c6-8dae-b180d1efae98",
          "started": "2023-10-16T23:56:27.365540Z",
          "expired": "2023-10-16T04:41:17.676340Z"
        }
      }
    }
  ]
  "###);
}
