use crate::database;
use crate::error::Result;
use bits_data::AuctionProductCreated;

pub fn auction_product_created(event: AuctionProductCreated) -> Result<()> {
  let auction_product = event.auction_product;

  database::db()
    .auction_products
    .insert(auction_product.id, auction_product);

  Ok(())
}
