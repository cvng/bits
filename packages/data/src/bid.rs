use crate::id;
use crate::Amount;
use crate::AuctionProductId;
use crate::DateTime;
use crate::UserId;
use async_graphql::SimpleObject;

id!(BidId);

#[derive(Clone, Copy, Serialize, SimpleObject)]
#[graphql(name = "BaseBid")]
pub struct Bid {
  pub id: BidId,
  pub user_id: UserId,
  pub product_id: AuctionProductId,
  pub amount: Amount,
  pub created_at: DateTime,
}
