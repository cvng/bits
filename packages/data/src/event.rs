use crate::Amount;
use crate::AuctionId;
use crate::BidId;
use crate::CommentId;
use crate::ProductId;
use crate::ShowId;
use crate::UserId;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionCreated { data: AuctionCreated },
  BidCreated { data: BidCreated },
  CommentCreated { data: CommentCreated },
  PersonCreated { data: PersonCreated },
  ProductCreated { data: ProductCreated },
  ShowCreated { data: ShowCreated },
}

impl Event {
  pub fn auction_created(
    id: AuctionId,
    show_id: ShowId,
    product_id: ProductId,
  ) -> Self {
    Self::AuctionCreated {
      data: AuctionCreated {
        id,
        show_id,
        product_id,
      },
    }
  }

  pub fn bid_created(
    id: BidId,
    auction_id: AuctionId,
    bidder_id: UserId,
    amount: Amount,
  ) -> Self {
    Self::BidCreated {
      data: BidCreated {
        id,
        auction_id,
        bidder_id,
        amount,
      },
    }
  }

  pub fn comment_created(
    id: CommentId,
    author_id: UserId,
    show_id: ShowId,
    text: String,
  ) -> Self {
    Self::CommentCreated {
      data: CommentCreated {
        id,
        author_id,
        show_id,
        text,
      },
    }
  }

  pub fn person_created(id: UserId, email: String, role: String) -> Self {
    Self::PersonCreated {
      data: PersonCreated { id, email, role },
    }
  }

  pub fn product_created(
    id: ProductId,
    creator_id: UserId,
    name: String,
  ) -> Self {
    Self::ProductCreated {
      data: ProductCreated {
        id,
        creator_id,
        name,
      },
    }
  }

  pub fn show_created(id: ShowId, creator_id: UserId, name: String) -> Self {
    Self::ShowCreated {
      data: ShowCreated {
        id,
        creator_id,
        name,
      },
    }
  }
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
  pub bidder_id: UserId,
  pub amount: Amount,
}

#[derive(Clone, Serialize)]
pub struct CommentCreated {
  pub id: CommentId,
  pub author_id: UserId,
  pub show_id: ShowId,
  pub text: String,
}

#[derive(Clone, Serialize)]
pub struct PersonCreated {
  pub id: UserId,
  pub email: String,
  pub role: String,
}

#[derive(Clone, Serialize)]
pub struct ProductCreated {
  pub id: ProductId,
  pub creator_id: UserId,
  pub name: String,
}

#[derive(Clone, Serialize)]
pub struct ShowCreated {
  pub id: ShowId,
  pub creator_id: UserId,
  pub name: String,
}
