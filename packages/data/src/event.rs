use crate::AuctionId;
use crate::AuctionProduct;
use crate::Bid;
use crate::Comment;
use crate::DateTime;
use crate::Product;
use crate::Show;
use crate::ShowId;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionMarkedReady(AuctionMarkedReady),
  AuctionProductCreated(AuctionProductCreated),
  AuctionRevived { payload: AuctionRevived },
  AuctionStarted(AuctionStarted),
  BidCreated { payload: BidCreated },
  CommentCreated { payload: CommentCreated },
  ProductCreated(ProductCreated),
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
}

#[derive(Clone, Serialize)]
pub struct AuctionMarkedReady {
  pub id: AuctionId,
  pub ready_at: DateTime,
}

#[derive(Clone, Serialize)]
pub struct AuctionProductCreated {
  pub auction_product: AuctionProduct,
}

#[derive(Clone, Serialize)]
pub struct AuctionRevived {
  pub id: AuctionId,
  pub expired_at: DateTime,
}

#[derive(Clone, Serialize)]
pub struct AuctionStarted {
  pub id: AuctionId,
  pub started_at: DateTime,
}

#[derive(Clone, Serialize)]
pub struct BidCreated {
  pub bid: Bid,
}

#[derive(Clone, Serialize)]
pub struct CommentCreated {
  pub comment: Comment,
}

#[derive(Clone, Serialize)]
pub struct ProductCreated {
  pub product: Product,
}

#[derive(Clone, Serialize)]
pub struct ShowCreated {
  pub show: Show,
}

#[derive(Clone, Serialize)]
pub struct ShowStarted {
  pub id: ShowId,
  pub started_at: DateTime,
}
