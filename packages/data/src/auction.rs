use crate::id;
use crate::DateTime;
use crate::ShowId;
use async_graphql::SimpleObject;

id!(AuctionId);

#[derive(Clone, Copy, SimpleObject)]
#[graphql(name = "BaseAuction")]
pub struct Auction {
  pub id: AuctionId,
  pub show_id: ShowId,
  pub ready_at: Option<DateTime>,
}
