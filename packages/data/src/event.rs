use crate::Amount;
use crate::AuctionId;
use crate::AuctionProductId;
use crate::BidId;
use crate::CommentId;
use crate::DateTime;
use crate::Product;
use crate::ProductId;
use crate::Show;
use crate::ShowId;
use crate::Text;
use crate::UserId;

pub enum Event {
  AuctionMarkedReady(AuctionMarkedReady),
  AuctionProductAdded(AuctionProductAdded),
  AuctionRevived(AuctionRevived),
  AuctionStarted(AuctionStarted),
  BidPlaced(BidPlaced),
  CommentAdded(CommentAdded),
  ProductCreated(ProductCreated),
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
}

pub struct AuctionMarkedReady {
  pub id: AuctionId,
  pub ready_at: DateTime,
}

pub struct AuctionProductAdded {
  pub id: AuctionProductId,
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

pub struct AuctionRevived {
  pub id: AuctionId,
  pub expired_at: DateTime,
}

pub struct AuctionStarted {
  pub id: AuctionId,
  pub started_at: DateTime,
}

pub struct BidPlaced {
  pub id: BidId,
  pub user_id: UserId,
  pub product_id: AuctionProductId,
  pub amount: Amount,
  pub created_at: DateTime,
}

pub struct CommentAdded {
  pub id: CommentId,
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

pub struct ProductCreated {
  pub product: Product,
}

pub struct ShowCreated {
  pub show: Show,
}

pub struct ShowStarted {
  pub id: ShowId,
  pub started_at: DateTime,
}
