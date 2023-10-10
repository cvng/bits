use crate::database;
use crate::error::Result;
use bits_data::AuctionProduct;
use bits_data::AuctionProductAdded;

pub fn auction_product_added(event: AuctionProductAdded) -> Result<()> {
  let auction_product = AuctionProduct {
    id: event.id,
    auction_id: event.auction_id,
    product_id: event.product_id,
  };

  database::db()
    .auction_products
    .insert(auction_product.id, auction_product);

  Ok(())
}
