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
  pub created: Option<DateTime>,
  pub updated: Option<DateTime>,
  pub auction_id: AuctionId,
  pub bidder_id: UserId,
  pub concurrent_amount: Option<Amount>,
  pub amount: Amount,
}
