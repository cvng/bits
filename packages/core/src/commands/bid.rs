use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionProduct;
use bits_data::AuctionProductId;
use bits_data::Bid;
use bits_data::BidId;
use bits_data::Duration;
use bits_data::Event;
use bits_data::UserId;
use bits_data::Utc;
use bits_data::AUCTION_REFRESH_SECS;
use thiserror::Error;

#[derive(InputObject)]
pub struct BidInput {
  pub user_id: UserId,
  pub product_id: AuctionProductId,
  pub amount: Amount,
}

#[derive(SimpleObject)]
pub struct BidResult {
  pub bid: Bid,
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
  ProductNotFound(AuctionProductId),
}

#[derive(Default)]
pub struct BidCommand {
  pub auction: Option<Auction>,
  pub product: Option<AuctionProduct>,
  pub best_bid: Option<Bid>,
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
    let product = self
      .product
      .ok_or(Error::ProductNotFound(input.product_id))?;

    let mut auction = self
      .auction
      .ok_or(Error::AuctionNotFound(product.auction_id))?;

    let bid = self.bid.ok_or(Error::NotCreated)?;

    auction.ready_at.ok_or(Error::AuctionNotReady(auction.id))?;

    auction
      .started_at
      .ok_or(Error::AuctionNotStarted(auction.id))?;

    let expired_at = auction
      .expired_at
      .ok_or(Error::AuctionNotStarted(auction.id))?;

    (bid.created_at < expired_at)
      .then_some(())
      .ok_or(Error::AuctionExpired(auction.id))?;

    self
      .best_bid
      .map_or_else(
        || Some(()),
        |best_bid| (bid.amount < best_bid.amount).then_some(()),
      )
      .ok_or(Error::InvalidAmount(bid.amount))?;

    auction.expired_at =
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

pub fn bid(input: BidInput) -> Result<BidResult, Error> {
  let product = database::db()
    .auction_products
    .get(&input.product_id)
    .cloned();

  let auction = product.and_then(|product| {
    database::db().auctions.get(&product.auction_id).cloned()
  });

  let best_bid = product.and_then(|product| {
    product
      .best_bid_id
      .and_then(|best_bid_id| database::db().bids.get(&best_bid_id).cloned())
  });

  let bid = Some(Bid {
    id: BidId::new(),
    user_id: input.user_id,
    product_id: input.product_id,
    amount: input.amount,
    created_at: Utc::now(),
  });

  BidCommand {
    auction,
    product,
    best_bid,
    bid,
  }
  .handle(input)
  .map(dispatcher::dispatch)?
  .map(BidCommand::apply)
  .map_err(|_| Error::NotCreated)?
  .ok_or(Error::NotCreated)
}

#[test]
fn test_bid() {
  let now = "2023-10-17T03:16:49.225067Z".parse().unwrap();

  let auction = Some(Auction {
    id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
    show_id: "28e9d842-0918-460f-9cd9-7245dbba1966".parse().unwrap(),
    ready_at: Some("2023-10-16T23:56:27.365540Z".parse().unwrap()),
    started_at: Some("2023-10-16T23:56:27.365540Z".parse().unwrap()),
    expired_at: Some(now + Duration::seconds(bits_data::AUCTION_TIMEOUT_SECS)),
  });

  let product = Some(bits_data::AuctionProduct {
    id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
    auction_id: auction.as_ref().unwrap().id,
    product_id: bits_data::ProductId::new(),
    best_bid_id: None,
    created_at: now,
  });

  let best_bid = None;

  let input = BidInput {
    user_id: "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68".parse().unwrap(),
    product_id: product.as_ref().unwrap().id,
    amount: 100,
  };

  let bid = Some(Bid {
    id: "bcd0ab01-96f0-4469-a3e6-254afe70b74f".parse().unwrap(),
    user_id: input.user_id,
    product_id: input.product_id,
    amount: input.amount,
    created_at: "2023-10-16T04:41:02.676340Z".parse().unwrap(),
  });

  let events = BidCommand {
    auction,
    product,
    best_bid,
    bid,
  }
  .handle(input)
  .unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "bid_created",
      "payload": {
        "bid": {
          "id": "bcd0ab01-96f0-4469-a3e6-254afe70b74f",
          "user_id": "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68",
          "product_id": "6bc8e88e-fc47-41c6-8dae-b180d1efae98",
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
          "ready_at": "2023-10-16T23:56:27.365540Z",
          "started_at": "2023-10-16T23:56:27.365540Z",
          "expired_at": "2023-10-16T04:41:17.676340Z"
        }
      }
    }
  ]
  "###);
}
