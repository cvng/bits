use crate::id;
use crate::ShowProductId;
use crate::UserId;
use async_graphql::SimpleObject;

id!(BidId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseBid")]
pub struct Bid {
  pub id: BidId,
  pub user_id: UserId,
  pub product_id: ShowProductId,
}
