use crate::id;
use crate::DateTime;
use async_graphql::SimpleObject;

id!(UserId);

#[derive(Clone, SimpleObject)]
pub struct User {
  pub id: UserId,
  pub created: Option<DateTime>,
  pub updated: Option<DateTime>,
  pub email: String,
}
