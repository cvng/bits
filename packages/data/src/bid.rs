use crate::Id;
use crate::ShowProductId;
use crate::UserId;
use async_graphql::SimpleObject;

pub type BidId = Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseBid")]
pub struct Bid {
    pub id: BidId,
    pub bidder_id: UserId,
    pub product_id: ShowProductId,
}
