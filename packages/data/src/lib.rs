#[macro_use]
extern crate serde;

pub mod entities;
mod event;
mod scalars;

pub use entities::auction::Model as Auction;
pub use entities::bid::Model as Bid;
pub use entities::comment::Model as Comment;
pub use entities::person::Model as User;
pub use entities::product::Model as Product;
pub use entities::show::Model as Show;
pub use event::*;
pub use scalars::*;

pub type ProductId = Uuid;
pub type AuctionId = Uuid;
pub type UserId = Uuid;
pub type CommentId = Uuid;
pub type BidId = Uuid;
pub type ShowId = Uuid;

pub const AUCTION_TIMEOUT_SECS: i64 = 60;
pub const AUCTION_REFRESH_SECS: i64 = 15;
