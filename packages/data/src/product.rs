use crate::id;
use async_graphql::SimpleObject;

id!(ProductId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseProduct")]
pub struct Product {
  pub id: ProductId,
}
