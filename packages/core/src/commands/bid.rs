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
pub struct BidPayload {
  pub bid: Bid,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("bid not found: {0}")]
  BidNotFound(BidId),
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
  #[error("product not found: {0}")]
  ProductNotFound(AuctionProductId),
  #[error("user not found: {0}")]
  UserNotFound(UserId),
}

#[derive(Default)]
struct BidCommand {
  auction: Option<Auction>,
  product: Option<AuctionProduct>,
  best_bid: Option<Bid>,
}

impl BidCommand {
  pub fn new(
    auction: Option<Auction>,
    product: Option<AuctionProduct>,
    best_bid: Option<Bid>,
  ) -> Self {
    Self {
      auction,
      product,
      best_bid,
    }
  }
}

impl Command for BidCommand {
  type Error = Error;
  type Event = Event;
  type Input = BidInput;
  type State = BidCommand;
  type Payload = BidPayload;

  fn handle(
    state: &Self::State,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let product = state
      .product
      .ok_or(Error::ProductNotFound(input.product_id))?;

    let auction = state
      .auction
      .ok_or(Error::AuctionNotFound(product.auction_id))?;

    let bid = Bid {
      id: BidId::new(),
      user_id: input.user_id,
      product_id: input.product_id,
      amount: input.amount,
      created_at: Utc::now(),
    };

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

    state
      .best_bid
      .map_or_else(
        || Some(()),
        |best_bid| (bid.amount < best_bid.amount).then_some(()),
      )
      .ok_or(Error::InvalidAmount(bid.amount))?;

    let expired_at = bid.created_at + Duration::seconds(AUCTION_REFRESH_SECS);

    Ok(vec![
      Event::bid_created(bid),
      Event::auction_revived(auction.id, expired_at),
    ])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Payload> {
    events.iter().fold(None, |_, event| match event {
      Event::BidCreated { payload } => Some(BidPayload { bid: payload.bid }),
      _ => None,
    })
  }
}

pub fn bid(input: BidInput) -> Result<BidPayload, Error> {
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

  let state = BidCommand::new(auction, product, best_bid);

  BidCommand::handle(&state, input)
    .map(|events| dispatcher::dispatch(events).unwrap())
    .map(|events| BidCommand::apply(events).unwrap())
}

#[test]
fn test_bid() {
  let now = Utc::now();

  let state = BidCommand {
    auction: Some(Auction {
      id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
      show_id: bits_data::ShowId::new(),
      ready_at: Some("2023-10-15T22:46:58.012577Z".parse().unwrap()),
      started_at: Some(now),
      expired_at: Some(
        now + Duration::seconds(bits_data::AUCTION_TIMEOUT_SECS),
      ),
    }),
    product: Some(bits_data::AuctionProduct {
      id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
      auction_id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
      product_id: bits_data::ProductId::new(),
      best_bid_id: None,
    }),
    best_bid: None,
  };

  let input = BidInput {
    user_id: "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68".parse().unwrap(),
    product_id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
    amount: 100,
  };

  let events = BidCommand::handle(&state, input).unwrap();

  assert_json_snapshot!(events, {
    "[0].payload.bid.id" => "[uuid]",
    "[0].payload.bid.created_at" => "[datetime]",
    "[1].payload.expired_at" => "[datetime]",
  }, @r###"
  [
    {
      "type": "bid_created",
      "payload": {
        "bid": {
          "id": "[uuid]",
          "user_id": "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68",
          "product_id": "6bc8e88e-fc47-41c6-8dae-b180d1efae98",
          "amount": 100,
          "created_at": "[datetime]"
        }
      }
    },
    {
      "type": "auction_revived",
      "payload": {
        "id": "f7223b3f-4045-4ef2-a8c3-058e1f742f2e",
        "expired_at": "[datetime]"
      }
    }
  ]
  "###);
}
