use crate::Auction;
use crate::Comment;
use crate::Product;
use crate::Show;
use crate::User;
use async_graphql::ComplexObject;

#[ComplexObject]
impl Show {
  async fn auction(&self) -> Option<Auction> {
    None
  }

  async fn comments(&self) -> Vec<Comment> {
    vec![]
  }

  async fn creator(&self) -> Option<User> {
    None
  }

  async fn products(&self) -> Vec<Product> {
    vec![]
  }
}
