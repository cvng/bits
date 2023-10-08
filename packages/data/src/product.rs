use crate::Id;
use async_graphql::SimpleObject;

pub type ProductId = Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseProduct")]
pub struct Product {
  pub id: ProductId,
}
