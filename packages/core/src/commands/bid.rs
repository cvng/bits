use crate::database;
use crate::dispatch::Command;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::Auction;
use bits_data::AuctionId;
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

struct State {
  pub auction: Auction,
  pub best_bid: Option<Bid>,
}

struct BidCommand;

impl Command for BidCommand {
  type Error = Error;
  type State = State;
  type Input = BidInput;
  type Payload = BidPayload;

  fn state(&self, input: &Self::Input) -> Result<Self::State, Self::Error> {
    let product = database::db()
      .auction_products
      .get(&input.product_id)
      .cloned()
      .ok_or(Error::ProductNotFound(input.product_id))?;

    let auction = database::db()
      .auctions
      .get(&product.auction_id)
      .cloned()
      .ok_or(Error::AuctionNotFound(product.auction_id))?;

    let best_bid = product
      .best_bid_id
      .and_then(|best_bid_id| database::db().bids.get(&best_bid_id).cloned());

    Ok(State { auction, best_bid })
  }

  fn events(
    &self,
    state: &Self::State,
    input: &Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    let bid = Bid {
      id: BidId::new(),
      user_id: input.user_id,
      product_id: input.product_id,
      amount: input.amount,
      created_at: Utc::now(),
    };

    state
      .auction
      .ready_at
      .ok_or(Error::AuctionNotReady(state.auction.id))?;

    state
      .auction
      .started_at
      .ok_or(Error::AuctionNotStarted(state.auction.id))?;

    let mut expired_at = state
      .auction
      .expired_at
      .ok_or(Error::AuctionNotStarted(state.auction.id))?;

    (bid.created_at < expired_at)
      .then_some(())
      .ok_or(Error::AuctionExpired(state.auction.id))?;

    state
      .best_bid
      .map_or_else(
        || Some(()),
        |best_bid| (bid.amount < best_bid.amount).then_some(()),
      )
      .ok_or(Error::InvalidAmount(bid.amount))?;

    expired_at += Duration::seconds(AUCTION_REFRESH_SECS);

    Ok(vec![
      Event::bid_created(bid),
      Event::auction_revived(state.auction.id, expired_at),
    ])
  }

  fn payload(&self, events: Vec<Event>) -> Option<Self::Payload> {
    events.iter().fold(None, |_, event| match event {
      Event::BidCreated { payload } => Some(BidPayload { bid: payload.bid }),
      _ => None,
    })
  }
}

pub fn bid(input: BidInput) -> Result<BidPayload, Error> {
  BidCommand.run(input)
}

#[test]
fn test_bid() {
  let state = State {
    auction: Auction {
      id: "f7223b3f-4045-4ef2-a8c3-058e1f742f2e".parse().unwrap(),
      show_id: bits_data::ShowId::new(),
      ready_at: Some(Utc::now()),
      started_at: Some(Utc::now()),
      expired_at: Some(
        Utc::now() + Duration::seconds(bits_data::AUCTION_TIMEOUT_SECS),
      ),
    },
    best_bid: None,
  };

  let input = BidInput {
    user_id: "0a0ccd87-2c7e-4dd6-b7d9-51d5a41c9c68".parse().unwrap(),
    product_id: "6bc8e88e-fc47-41c6-8dae-b180d1efae98".parse().unwrap(),
    amount: 100,
  };

  let events = BidCommand.events(&state, &input).unwrap();

  assert_json_snapshot!(events, {
    "[0].payload.bid.id" => "[uuid]",
    "[0].payload.bid.created_at" => "[datetime]",
    "[1].payload.expired_at" => "[datetime]"
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
