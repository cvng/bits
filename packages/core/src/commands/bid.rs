use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Amount;
use bits_data::AuctionProductId;
use bits_data::Bid;
use bits_data::BidId;
use bits_data::BidPlaced;
use bits_data::UserId;
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

#[derive(Error, Debug)]
pub enum Error {
  #[error("product not found: {0}")]
  ProductNotFound(AuctionProductId),
}

pub async fn bid(input: BidInput) -> Result<BidPayload, Error> {
  database::db()
    .auction_products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  let bid = Bid {
    id: BidId::new(),
    user_id: input.user_id,
    product_id: input.product_id,
    amount: input.amount,
  };

  dispatch::dispatch(vec![BidPlaced {
    id: bid.id,
    user_id: bid.user_id,
    product_id: bid.product_id,
    amount: bid.amount,
  }
  .into()])
  .ok();

  Ok(BidPayload { bid })
}
