use crate::id;
use crate::Text;
use async_graphql::SimpleObject;

id!(ProductId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Product {
  pub id: ProductId,
  pub name: Text,
}
