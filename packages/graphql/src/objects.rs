use async_graphql::ComplexObject;
use async_graphql::SimpleObject;
use bits_core as bits_data;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Auction {
  #[graphql(flatten)]
  inner: bits_data::Auction,
}

#[ComplexObject]
impl Auction {
  async fn bids(&self) -> Vec<Bid> {
    vec![]
  }
}

#[derive(SimpleObject)]
pub struct Bid {
  #[graphql(flatten)]
  inner: bits_data::Bid,
}

#[derive(SimpleObject)]
pub struct Comment {
  #[graphql(flatten)]
  inner: bits_data::Comment,
}

#[derive(SimpleObject)]
pub struct Product {
  #[graphql(flatten)]
  inner: bits_data::Product,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Show {
  #[graphql(flatten)]
  inner: bits_data::Show,
}

#[ComplexObject]
impl Show {
  async fn auction(&self) -> Option<Auction> {
    None
  }

  async fn comments(&self) -> Vec<Comment> {
    vec![]
  }

  async fn creator(&self) -> User {
    bits_data::User {
      id: self.inner.creator_id,
    }
    .into()
  }

  async fn products(&self) -> Vec<Product> {
    vec![]
  }
}

impl From<bits_data::Show> for Show {
  fn from(inner: bits_data::Show) -> Self {
    Self { inner }
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
