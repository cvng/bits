use async_graphql::SimpleObject;

pub type UserId = crate::Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseUser")]
pub struct User {
    pub id: UserId,
}
