use crate::id;
use crate::Amount;
use crate::AuctionId;
use crate::DateTime;
use crate::UserId;
use async_graphql::SimpleObject;

id!(BidId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Bid {
  pub id: BidId,
  pub auction_id: AuctionId,
  pub bidder_id: UserId,
  pub amount: Amount,
  pub created_at: DateTime,
}
