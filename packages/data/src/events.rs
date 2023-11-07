use crate::scalars::amount::Amount;
use crate::types::AuctionId;
use crate::types::BidId;
use crate::types::CommentId;
use crate::types::PersonId;
use crate::types::ProductId;
use crate::types::ShowId;
use crate::Show;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionCreated { data: AuctionCreated },
  BidCreated { data: BidCreated },
  CommentCreated { data: CommentCreated },
  PersonCreated { data: PersonCreated },
  ProductCreated { data: ProductCreated },
  ShowCreated { data: ShowCreated },
  ShowStarted { data: ShowStarted },
}

#[derive(Clone, Serialize)]
pub struct AuctionCreated {
  pub id: AuctionId,
  pub show_id: ShowId,
  pub product_id: ProductId,
}

#[derive(Clone, Serialize)]
pub struct BidCreated {
  pub id: BidId,
  pub auction_id: AuctionId,
  pub bidder_id: PersonId,
  pub amount: Amount,
}

#[derive(Clone, Serialize)]
pub struct CommentCreated {
  pub id: CommentId,
  pub author_id: PersonId,
  pub show_id: ShowId,
  pub text: String,
}

#[derive(Clone, Serialize)]
pub struct PersonCreated {
  pub id: PersonId,
  pub email: String,
  pub role: String,
}

#[derive(Clone, Serialize)]
pub struct ProductCreated {
  pub id: ProductId,
  pub creator_id: PersonId,
  pub name: String,
}

#[derive(Clone, Serialize)]
pub struct ShowCreated {
  pub id: ShowId,
  pub creator_id: PersonId,
  pub name: String,
}

#[derive(Clone, Serialize)]
pub struct ShowStarted {
  pub id: ShowId,
  pub show: Show,
}
