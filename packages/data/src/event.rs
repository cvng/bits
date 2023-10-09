use crate::DateTime;
use crate::Product;
use crate::ProductId;
use crate::Show;
use crate::ShowId;
use crate::ShowProductId;

pub enum Event {
  ProductCreated(ProductCreated),
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
  ShowProductAdded(ShowProductAdded),
}

pub struct ProductCreated {
  pub product: Product,
}

impl From<ProductCreated> for Event {
  fn from(event: ProductCreated) -> Self {
    Self::ProductCreated(event)
  }
}

pub struct ShowCreated {
  pub show: Show,
}

impl From<ShowCreated> for Event {
  fn from(event: ShowCreated) -> Self {
    Self::ShowCreated(event)
  }
}

pub struct ShowStarted {
  pub id: ShowId,
  pub started_at: DateTime,
}

impl From<ShowStarted> for Event {
  fn from(event: ShowStarted) -> Self {
    Self::ShowStarted(event)
  }
}

pub struct ShowProductAdded {
  pub id: ShowProductId,
  pub show_id: ShowId,
  pub product_id: ProductId,
}

impl From<ShowProductAdded> for Event {
  fn from(event: ShowProductAdded) -> Self {
    Self::ShowProductAdded(event)
  }
}
