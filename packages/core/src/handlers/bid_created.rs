use crate::database;
use crate::error::Result;
use crate::Error;
use bits_data::BidCreated;

pub fn bid_created(event: BidCreated) -> Result<()> {
  let bid = event.bid;

  database::db().bids.insert(bid.id, bid);

  let mut auction_product = database::db()
    .auction_products
    .values()
    .find(|auction_product| auction_product.id == bid.product_id)
    .cloned()
    .ok_or(Error::NotFound(bid.product_id.into()))?;

  auction_product.best_bid_id = Some(bid.id);

  database::db()
    .auction_products
    .insert(auction_product.id, auction_product);

  Ok(())
}
