use crate::AuctionId;
use crate::AuctionProduct;
use crate::Bid;
use crate::Comment;
use crate::DateTime;
use crate::Product;
use crate::Show;
use crate::ShowId;

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

pub struct AuctionMarkedReady {
  pub id: AuctionId,
  pub ready_at: DateTime,
}

pub struct AuctionProductCreated {
  pub auction_product: AuctionProduct,
}

pub struct AuctionRevived {
  pub id: AuctionId,
  pub expired_at: DateTime,
}

pub struct AuctionStarted {
  pub id: AuctionId,
  pub started_at: DateTime,
}

pub struct BidCreated {
  pub bid: Bid,
}

pub struct CommentCreated {
  pub comment: Comment,
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
