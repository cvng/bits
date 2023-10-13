use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::AuctionId;
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

#[derive(Error, Debug)]
pub enum Error {
  #[error("auction not found: {0}")]
  AuctionNotFound(AuctionId),
  #[error("auction not ready: {0}")]
  AuctionNotReady(AuctionId),
  #[error("auction expired: {0}")]
  AuctionExpired(AuctionId),
  #[error("auction not expired: {0}")]
  AuctionNotExpired(AuctionId),
  #[error("product not found: {0}")]
  ProductNotFound(AuctionProductId),
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
