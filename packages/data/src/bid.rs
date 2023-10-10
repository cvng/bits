use crate::id;
use crate::Amount;
use crate::AuctionProductId;
use crate::UserId;
use async_graphql::SimpleObject;

id!(BidId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseBid")]
pub struct Bid {
  pub id: BidId,
  pub user_id: UserId,
  pub product_id: AuctionProductId,
  pub amount: Amount,
}
