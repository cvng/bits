use async_graphql::Object;
use bits_core::db;
use bits_core::Show;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn shows(&self) -> Vec<Show> {
    db().shows.values().cloned().collect()
  }
}
