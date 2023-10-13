use crate::AuctionId;
use crate::AuctionProduct;
use crate::Bid;
use crate::Comment;
use crate::DateTime;
use crate::Product;
use crate::Show;
use crate::ShowId;

#[derive(Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
  AuctionMarkedReady(AuctionMarkedReady),
  AuctionProductCreated(AuctionProductCreated),
  AuctionRevived(AuctionRevived),
  AuctionStarted(AuctionStarted),
  BidCreated(BidCreated),
  CommentCreated(CommentCreated),
  ProductCreated(ProductCreated),
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
}

#[derive(Serialize)]
pub struct AuctionMarkedReady {
  pub id: AuctionId,
  pub ready_at: DateTime,
}

#[derive(Serialize)]
pub struct AuctionProductCreated {
  pub auction_product: AuctionProduct,
}

#[derive(Serialize)]
pub struct AuctionRevived {
  pub id: AuctionId,
  pub expired_at: DateTime,
}

#[derive(Serialize)]
pub struct AuctionStarted {
  pub id: AuctionId,
  pub started_at: DateTime,
}

#[derive(Serialize)]
pub struct BidCreated {
  pub bid: Bid,
}

#[derive(Serialize)]
pub struct CommentCreated {
  pub comment: Comment,
}

#[derive(Serialize)]
pub struct ProductCreated {
  pub product: Product,
}

#[derive(Serialize)]
pub struct ShowCreated {
  pub show: Show,
}

#[derive(Serialize)]
pub struct ShowStarted {
  pub id: ShowId,
  pub started_at: DateTime,
}
