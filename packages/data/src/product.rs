use crate::id;
use crate::DateTime;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(ProductId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Product {
  pub id: ProductId,
  pub created: Option<DateTime>,
  pub updated: Option<DateTime>,
  pub creator_id: UserId,
  pub name: Text,
}
