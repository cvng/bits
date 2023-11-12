use crate::scalars::amount::Amount;
use crate::types::AuctionId;
use crate::types::BidId;
use crate::types::CommentId;
use crate::types::PersonId;
use crate::types::ProductId;
use crate::types::ShowId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionCreated { data: AuctionCreated },
  AuctionStarted { data: AuctionStarted },
  BidCreated { data: BidCreated },
  CommentCreated { data: CommentCreated },
  PersonCreated { data: PersonCreated },
  ProductCreated { data: ProductCreated },
  ShowCreated { data: ShowCreated },
  ShowStarted { data: ShowStarted },
}

#[derive(Clone, Debug, Serialize)]
pub struct AuctionCreated {
  pub id: AuctionId,
  pub show_id: ShowId,
  pub product_id: ProductId,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuctionStarted {
  pub id: AuctionId,
}

#[derive(Clone, Debug, Serialize)]
pub struct BidCreated {
  pub id: BidId,
  pub auction_id: AuctionId,
  pub buyer_id: PersonId,
  pub amount: Amount,
}

#[derive(Clone, Debug, Serialize)]
pub struct CommentCreated {
  pub id: CommentId,
  pub author_id: PersonId,
  pub show_id: ShowId,
  pub text: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PersonCreated {
  pub id: PersonId,
  pub email: String,
  pub role: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductCreated {
  pub id: ProductId,
  pub creator_id: PersonId,
  pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ShowCreated {
  pub id: ShowId,
  pub creator_id: PersonId,
  pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ShowStarted {
  pub id: ShowId,
}
