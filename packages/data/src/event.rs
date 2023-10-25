use crate::Auction;
use crate::Bid;
use crate::Comment;
use crate::Product;
use crate::Show;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionCreated { payload: AuctionCreated },
  AuctionRevived { payload: AuctionRevived },
  AuctionStarted { payload: AuctionStarted },
  BidCreated { payload: BidCreated },
  CommentCreated { payload: CommentCreated },
  ProductCreated { payload: ProductCreated },
  ShowCreated { payload: ShowCreated },
  ShowStarted { payload: ShowStarted },
}

impl Event {
  pub fn auction_created(auction: Auction) -> Self {
    Self::AuctionCreated {
      payload: AuctionCreated { auction },
    }
  }

  pub fn auction_revived(auction: Auction) -> Self {
    Self::AuctionRevived {
      payload: AuctionRevived { auction },
    }
  }

  pub fn auction_started(auction: Auction) -> Self {
    Self::AuctionStarted {
      payload: AuctionStarted { auction },
    }
  }

  pub fn bid_created(bid: Bid) -> Self {
    Self::BidCreated {
      payload: BidCreated { bid },
    }
  }

  pub fn comment_created(comment: Comment) -> Self {
    Self::CommentCreated {
      payload: CommentCreated { comment },
    }
  }

  pub fn product_created(product: Product) -> Self {
    Self::ProductCreated {
      payload: ProductCreated { product },
    }
  }

  pub fn show_created(show: Show) -> Self {
    Self::ShowCreated {
      payload: ShowCreated { show },
    }
  }

  pub fn show_started(show: Show) -> Self {
    Self::ShowStarted {
      payload: ShowStarted { show },
    }
  }
}

#[derive(Clone, Serialize)]
pub struct AuctionMarkedReady {
  pub auction: Auction,
}

#[derive(Clone, Serialize)]
pub struct AuctionCreated {
  pub auction: Auction,
}

#[derive(Clone, Serialize)]
pub struct AuctionRevived {
  pub auction: Auction,
}

#[derive(Clone, Serialize)]
pub struct AuctionStarted {
  pub auction: Auction,
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
  pub show: Show,
}
