use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::ProductCreated;

pub fn product_created(event: ProductCreated) -> Result<()> {
  database::db()
    .products
    .insert(event.product.id, event.product)
    .map(|_| ())
    .ok_or(Error::NotFound(event.product.id.into()))
}
