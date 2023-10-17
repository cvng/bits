use crate::Auction;
use crate::Bid;
use async_graphql::ComplexObject;

#[ComplexObject]
impl Auction {
  async fn bids(&self) -> Vec<Bid> {
    vec![]
  }
}
