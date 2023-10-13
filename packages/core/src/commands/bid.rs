use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::AuctionId;
use bits_data::AuctionProduct;
use bits_data::AuctionProductId;
use bits_data::AuctionRevived;
use bits_data::Bid;
use bits_data::BidCreated;
use bits_data::BidId;
use bits_data::Duration;
use bits_data::Event;
use bits_data::User;
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

#[derive(Error, Debug)]
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

pub async fn bid(input: BidInput) -> Result<BidPayload, Error> {
  let product = database::db()
    .auction_products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  let mut auction = database::db()
    .auctions
    .get(&product.auction_id)
    .cloned()
    .ok_or(Error::AuctionNotFound(product.auction_id))?;

  if auction.ready_at.is_none() {
    return Err(Error::AuctionNotReady(auction.id));
  }

  if let Some(started_at) = auction.started_at {
    if started_at - Utc::now() < Duration::seconds(AUCTION_TIMEOUT_SECS) {
      return Err(Error::AuctionExpired(auction.id));
    }
  }

  let Some(expired_at) = auction.expired_at else {
    return Err(Error::AuctionNotExpired(auction.id));
  };

  if let Some(best_bid_id) = product.best_bid_id {
    let best_bid = database::db()
      .bids
      .get(&best_bid_id)
      .cloned()
      .ok_or(Error::BidNotFound(best_bid_id))?;

    if input.amount <= best_bid.amount {
      return Err(Error::InvalidBid);
    }
  }

  let bid = Bid {
    id: BidId::new(),
    user_id: input.user_id,
    product_id: input.product_id,
    amount: input.amount,
    created_at: Utc::now(),
  };

  let expired_at = expired_at + Duration::seconds(AUCTION_REFRESH_SECS);

  auction.expired_at = Some(expired_at);

  dispatch::dispatch(vec![
    Event::BidCreated(BidCreated { bid }),
    Event::AuctionRevived(AuctionRevived {
      id: auction.id,
      expired_at,
    }),
  ])
  .ok();

  Ok(BidPayload { bid })
}

fn _auction_product_winner(auction_product: &AuctionProduct) -> Option<User> {
  let Some(best_bid_id) = auction_product.best_bid_id else {
    return None;
  };

  let best_bid = database::db().bids.get(&best_bid_id).cloned();

  let Some(best_bid) = best_bid else {
    return None;
  };

  database::db().users.get(&best_bid.user_id).cloned()
}
