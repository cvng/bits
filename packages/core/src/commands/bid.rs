use crate::database;
use crate::dispatch::Command;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionProduct;
use bits_data::AuctionProductId;
use bits_data::AuctionRevived;
use bits_data::Bid;
use bits_data::BidCreated;
use bits_data::BidId;
use bits_data::Duration;
use bits_data::Event;
use bits_data::UserId;
use bits_data::Utc;
use thiserror::Error;

const AUCTION_TIMEOUT_SECS: i64 = 60;
const AUCTION_REFRESH_SECS: i64 = 15;

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
  #[error("auction expired: {0}")]
  AuctionExpired(AuctionId),
  #[error("auction not expired: {0}")]
  AuctionNotExpired(AuctionId),
  #[error("invalid bid")]
  InvalidBid,
  #[error("product not found: {0}")]
  ProductNotFound(AuctionProductId),
  #[error("user not found: {0}")]
  UserNotFound(UserId),
}

struct State {
  pub auction: Auction,
  pub product: AuctionProduct,
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

    Ok(State { product, auction })
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

    if state.auction.ready_at.is_none() {
      return Err(Error::AuctionNotReady(state.auction.id));
    }

    if let Some(started_at) = state.auction.started_at {
      if started_at - Utc::now() < Duration::seconds(AUCTION_TIMEOUT_SECS) {
        return Err(Error::AuctionExpired(state.auction.id));
      }
    }

    if let Some(best_bid_id) = state.product.best_bid_id {
      let best_bid = database::db()
        .bids
        .get(&best_bid_id)
        .cloned()
        .ok_or(Error::BidNotFound(best_bid_id))?;

      if bid.amount <= best_bid.amount {
        return Err(Error::InvalidBid);
      }
    }

    let Some(expired_at) = state.auction.expired_at else {
      return Err(Error::AuctionNotExpired(state.auction.id));
    };

    let expired_at = expired_at + Duration::seconds(AUCTION_REFRESH_SECS);

    Ok(vec![
      Event::BidCreated {
        payload: BidCreated { bid },
      },
      Event::AuctionRevived {
        payload: AuctionRevived {
          id: state.auction.id,
          expired_at,
        },
      },
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
