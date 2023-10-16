use bits_data as data;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::PoisonError;
use thiserror::Error;

static DATABASE: OnceLock<Mutex<DatabaseInner>> = OnceLock::new();

pub type Database = MutexGuard<'static, DatabaseInner>;

#[derive(Debug, Error)]
pub enum DatabaseError {
  #[error("database lock poisoned")]
  Lock(#[from] PoisonError<Database>),
}

#[derive(Default)]
pub struct DatabaseInner {
  pub auctions: HashMap<data::AuctionId, data::Auction>,
  pub auction_products: HashMap<data::AuctionProductId, data::AuctionProduct>,
  pub bids: HashMap<data::BidId, data::Bid>,
  pub comments: HashMap<data::CommentId, data::Comment>,
  pub shows: HashMap<data::ShowId, data::Show>,
  pub products: HashMap<data::ProductId, data::Product>,
  pub users: HashMap<data::UserId, data::User>,
}

pub fn db() -> Database {
  DATABASE
    .get_or_init(|| Mutex::new(DatabaseInner::default()))
    .lock()
    .map_err(DatabaseError::Lock)
    .unwrap()
}
