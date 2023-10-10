use crate::database;
use crate::error::Result;
use bits_data::ShowProduct;
use bits_data::ShowProductAdded;

pub fn show_product_added(event: ShowProductAdded) -> Result<()> {
  let show_product = ShowProduct {
    id: event.id,
    auction_id: event.auction_id,
    product_id: event.product_id,
  };

  database::db()
    .show_products
    .insert(show_product.id, show_product);

  Ok(())
}
