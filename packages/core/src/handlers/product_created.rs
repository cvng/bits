use crate::database;
use crate::error::Result;
use bits_data::ProductCreated;

pub fn product_created(event: ProductCreated) -> Result<()> {
  let product = event.product;

  database::db().products.insert(product.id, product);

  Ok(())
}
