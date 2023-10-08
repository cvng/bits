use crate::objects::Show;
use async_graphql::Object;
use bits_core::db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn shows(&self) -> Vec<Show> {
        db().shows.values().cloned().map(Into::into).collect()
    }
}
