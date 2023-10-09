use crate::id;
use async_graphql::SimpleObject;

id!(UserId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseUser")]
pub struct User {
  pub id: UserId,
}
