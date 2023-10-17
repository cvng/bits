use async_graphql::Object;
use bits_core::db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn bids(&self) -> Vec<bits_core::Bid> {
    db().bids.values().cloned().map(Into::into).collect()
  }

  async fn comments(&self) -> Vec<bits_core::Comment> {
    db().comments.values().cloned().map(Into::into).collect()
  }

  async fn shows(&self) -> Vec<bits_core::Show> {
    db().shows.values().cloned().map(Into::into).collect()
  }
}
