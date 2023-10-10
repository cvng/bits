use crate::objects;
use async_graphql::Object;
use bits_core::db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn bids(&self) -> Vec<objects::Bid> {
    db().bids.values().cloned().map(Into::into).collect()
  }

  async fn comments(&self) -> Vec<objects::Comment> {
    db().comments.values().cloned().map(Into::into).collect()
  }

  async fn shows(&self) -> Vec<objects::Show> {
    db().shows.values().cloned().map(Into::into).collect()
  }
}
