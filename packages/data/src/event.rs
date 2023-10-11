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

#[derive(Clone)]
pub enum Event {
  AuctionMarkedReady(AuctionMarkedReady),
  AuctionProductAdded(AuctionProductAdded),
  BidPlaced(BidPlaced),
  CommentAdded(CommentAdded),
  ProductCreated(ProductCreated),
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
}

#[derive(Clone)]
pub struct AuctionMarkedReady {
  pub id: AuctionId,
  pub ready_at: DateTime,
}

impl From<AuctionMarkedReady> for Event {
  fn from(event: AuctionMarkedReady) -> Self {
    Self::AuctionMarkedReady(event)
  }
}

#[derive(Clone)]
pub struct BidPlaced {
  pub id: BidId,
  pub user_id: UserId,
  pub product_id: AuctionProductId,
  pub amount: Amount,
}

impl From<BidPlaced> for Event {
  fn from(event: BidPlaced) -> Self {
    Self::BidPlaced(event)
  }
}

#[derive(Clone)]
pub struct CommentAdded {
  pub id: CommentId,
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

impl From<CommentAdded> for Event {
  fn from(event: CommentAdded) -> Self {
    Self::CommentAdded(event)
  }
}

#[derive(Clone)]
pub struct ProductCreated {
  pub product: Product,
}

impl From<ProductCreated> for Event {
  fn from(event: ProductCreated) -> Self {
    Self::ProductCreated(event)
  }
}

#[derive(Clone)]
pub struct ShowCreated {
  pub show: Show,
}

impl From<ShowCreated> for Event {
  fn from(event: ShowCreated) -> Self {
    Self::ShowCreated(event)
  }
}

#[derive(Clone)]
pub struct ShowStarted {
  pub id: ShowId,
  pub started_at: DateTime,
}

impl From<ShowStarted> for Event {
  fn from(event: ShowStarted) -> Self {
    Self::ShowStarted(event)
  }
}

#[derive(Clone)]
pub struct AuctionProductAdded {
  pub id: AuctionProductId,
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

impl From<AuctionProductAdded> for Event {
  fn from(event: AuctionProductAdded) -> Self {
    Self::AuctionProductAdded(event)
  }
}
