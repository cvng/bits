use crate::id;
use crate::AuctionId;
use crate::DateTime;
use crate::ProductId;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(ShowId);
id!(ShowProductId);

#[derive(Clone, Copy, SimpleObject)]
#[graphql(name = "BaseShow")]
pub struct Show {
  pub id: ShowId,
  pub creator_id: UserId,
  pub name: Text,
  pub started_at: Option<DateTime>,
}

#[derive(Clone, SimpleObject)]
pub struct ShowProduct {
  pub id: ShowProductId,
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}
