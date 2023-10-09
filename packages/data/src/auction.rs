use crate::id;
use crate::ShowId;
use async_graphql::SimpleObject;

id!(AuctionId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseAuction")]
pub struct Auction {
  pub id: AuctionId,
  pub show_id: ShowId,
}
