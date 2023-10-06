use async_graphql::ComplexObject;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Show {
    #[graphql(flatten)]
    inner: bits_data::Show,
}

#[ComplexObject]
impl Show {
    async fn creator(&self) -> User {
        bits_data::User {
            id: self.inner.creator_id.clone(),
        }
        .into()
    }
}

#[derive(SimpleObject)]
pub struct User {
    #[graphql(flatten)]
    inner: bits_data::User,
}

impl From<bits_data::User> for User {
    fn from(inner: bits_data::User) -> Self {
        Self { inner }
    }
}
