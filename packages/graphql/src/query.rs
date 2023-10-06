use crate::objects::Show;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn shows(&self) -> Vec<Show> {
        vec![] // TODO: implement
    }
}
