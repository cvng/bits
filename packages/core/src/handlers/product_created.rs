use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::ProductCreated;

pub async fn product_created(
  _client: &Client,
  event: ProductCreated,
) -> Result<()> {
  database::db()
    .products
    .insert(event.product.id, event.product.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.product.id))
}
