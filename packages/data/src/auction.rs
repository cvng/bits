use crate::ShowId;
use async_graphql::SimpleObject;

pub type AuctionId = crate::Id;

#[derive(Clone, SimpleObject)]
pub struct Auction {
    pub id: AuctionId,
    pub show_id: ShowId,
}
