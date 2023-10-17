use crate::id;
use async_graphql::SimpleObject;

id!(UserId);

#[derive(Clone, SimpleObject)]
pub struct User {
  pub id: UserId,
}
