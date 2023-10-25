use crate::id;
use crate::DateTime;
use crate::ProductId;
use crate::ShowId;
use async_graphql::SimpleObject;

pub const AUCTION_TIMEOUT_SECS: i64 = 60;
pub const AUCTION_REFRESH_SECS: i64 = 15;

id!(AuctionId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Auction {
  pub id: AuctionId,
  pub show_id: ShowId,
  pub product_id: ProductId,
  pub started: Option<DateTime>,
  pub expired: Option<DateTime>,
}
