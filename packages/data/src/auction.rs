use crate::Id;
use crate::ShowId;
use async_graphql::SimpleObject;

pub type AuctionId = Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseAuction")]
pub struct Auction {
  pub id: AuctionId,
  pub show_id: ShowId,
}
