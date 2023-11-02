use crate::Amount;
use crate::Auction;
use crate::AuctionId;
use crate::Bid;
use crate::BidId;
use crate::Comment;
use crate::CommentId;
use crate::Product;
use crate::ProductId;
use crate::Show;
use crate::ShowId;
use crate::UserId;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  AuctionCreated { payload: AuctionCreated },
  BidCreated { payload: BidCreated },
  CommentCreated { payload: CommentCreated },
  PersonCreated { payload: PersonCreated },
  ProductCreated { payload: ProductCreated },
  ShowCreated { payload: ShowCreated },
}

impl Event {
  pub fn auction_created(auction: Auction) -> Self {
    Self::AuctionCreated {
      payload: AuctionCreated {
        auction: auction.clone(),
        id: auction.id,
        show_id: auction.show_id,
        product_id: auction.product_id,
      },
    }
  }

  pub fn bid_created(bid: Bid) -> Self {
    Self::BidCreated {
      payload: BidCreated {
        bid: bid.clone(),
        id: bid.id,
        auction_id: bid.auction_id,
        bidder_id: bid.bidder_id,
        amount: bid.amount,
      },
    }
  }

  pub fn comment_created(comment: Comment) -> Self {
    Self::CommentCreated {
      payload: CommentCreated {
        comment: comment.clone(),
        id: comment.id,
        author_id: comment.author_id,
        show_id: comment.show_id,
        text: comment.text,
      },
    }
  }

  pub fn person_created(id: UserId, email: String, role: String) -> Self {
    Self::PersonCreated {
      payload: PersonCreated { id, email, role },
    }
  }

  pub fn product_created(product: Product) -> Self {
    Self::ProductCreated {
      payload: ProductCreated {
        product: product.clone(),
        id: product.id,
        creator_id: product.creator_id,
        name: product.name,
      },
    }
  }

  pub fn show_created(show: Show) -> Self {
    Self::ShowCreated {
      payload: ShowCreated {
        show: show.clone(),
        id: show.id,
        creator_id: show.creator_id,
        name: show.name,
      },
    }
  }
}

#[derive(Clone, Serialize)]
pub struct AuctionCreated {
  pub auction: Auction,
  //
  pub id: AuctionId,
  pub show_id: ShowId,
  pub product_id: ProductId,
}

#[derive(Clone, Serialize)]
pub struct BidCreated {
  pub bid: Bid,
  //
  pub id: BidId,
  pub auction_id: AuctionId,
  pub bidder_id: UserId,
  pub amount: Amount,
}

#[derive(Clone, Serialize)]
pub struct CommentCreated {
  pub comment: Comment,
  //
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
  pub product: Product,
  //
  pub id: ProductId,
  pub creator_id: UserId,
  pub name: String,
}

#[derive(Clone, Serialize)]
pub struct ShowCreated {
  pub show: Show,
  //
  pub id: ShowId,
  pub creator_id: UserId,
  pub name: String,
}
