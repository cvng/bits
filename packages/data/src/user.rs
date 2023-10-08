use crate::Id;
use async_graphql::SimpleObject;

pub type UserId = Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseUser")]
pub struct User {
    pub id: UserId,
}
