use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::AuctionProductCreated;

pub fn auction_product_created(event: AuctionProductCreated) -> Result<()> {
  database::db()
    .auction_products
    .insert(event.auction_product.id, event.auction_product)
    .map(|_| ())
    .ok_or(Error::NotFound(event.auction_product.id.into()))
}
