use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Product;
use bits_data::ProductCreated;
use bits_data::ProductId;
use bits_data::Text;
use thiserror::Error;

#[derive(InputObject)]
pub struct CreateProductInput {
  pub name: Text,
}

#[derive(SimpleObject)]
pub struct CreateProductPayload {
  pub product: Product,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ProductId),
}

pub async fn create_product(
  input: CreateProductInput,
) -> Result<CreateProductPayload, Error> {
  let product = Product {
    id: ProductId::new(),
    name: input.name,
  };

  dispatch::dispatch(vec![ProductCreated { product }.into()]).ok();

  Ok(CreateProductPayload {
    product: database::db()
      .products
      .get(&product.id)
      .cloned()
      .ok_or(Error::NotFound(product.id))?,
  })
}