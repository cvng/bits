use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::BidCreated;

pub fn bid_created(event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid)
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id.into()))?;

  let mut auction_product = database::db()
    .auction_products
    .values()
    .find(|auction_product| auction_product.id == event.bid.product_id)
    .cloned()
    .ok_or(Error::NotFound(event.bid.product_id.into()))?;

  auction_product.best_bid_id = Some(event.bid.id);

  database::db()
    .auction_products
    .insert(auction_product.id, auction_product)
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id.into()))
}
