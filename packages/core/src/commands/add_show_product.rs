use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::ShowMarkedReady;
use bits_data::ShowProduct;
use bits_data::ShowProductAdded;
use bits_data::ShowProductId;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct AddShowProductInput {
  pub show_id: ShowId,
  pub product_id: ProductId,
}

#[derive(SimpleObject)]
pub struct AddShowProductPayload {
  pub show: Show,
  pub product: Product,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
  #[error("product not found: {0}")]
  ProductNotFound(ProductId),
}

pub async fn add_show_product(
  input: AddShowProductInput,
) -> Result<AddShowProductPayload, Error> {
  let show = database::db()
    .shows
    .get(&input.show_id)
    .cloned()
    .ok_or(Error::ShowNotFound(input.show_id))?;

  let product = database::db()
    .products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  let show_product = ShowProduct {
    id: ShowProductId::new(),
    show_id: show.id,
    product_id: product.id,
  };

  let mut events = vec![ShowProductAdded {
    id: show_product.id,
    show_id: show_product.show_id,
    product_id: show_product.product_id,
  }
  .into()];

  if show.ready_at.is_none() {
    events.push(
      ShowMarkedReady {
        id: show.id,
        ready_at: Utc::now(),
      }
      .into(),
    )
  }

  dispatch::dispatch(events).ok();

  Ok(AddShowProductPayload { show, product })
}
