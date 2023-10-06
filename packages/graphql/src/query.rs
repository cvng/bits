use crate::objects::Show;
use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn shows(&self) -> Vec<Show> {
        vec![] // TODO: implement
    }
}
