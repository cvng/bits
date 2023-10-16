use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Text;
use thiserror::Error;

#[derive(InputObject)]
pub struct CreateProductInput {
  pub name: Text,
}

#[derive(SimpleObject)]
pub struct CreateProductResult {
  pub product: Product,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ProductId),
}

pub async fn create_product(
  input: CreateProductInput,
) -> Result<CreateProductResult, Error> {
  let product = Product {
    id: ProductId::new(),
    name: input.name,
  };

  dispatcher::dispatch(vec![Event::product_created(product)]).ok();

  Ok(CreateProductResult { product })
}
