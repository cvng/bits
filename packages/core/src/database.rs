use bits_data as data;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::PoisonError;
use thiserror::Error;

static DATABASE: OnceLock<Mutex<Database>> = OnceLock::new();

pub type DatabaseGuard = MutexGuard<'static, Database>;

#[derive(Debug, Error)]
pub enum DatabaseError {
  #[error("database lock poisoned")]
  Lock(#[from] PoisonError<DatabaseGuard>),
}

#[derive(Default)]
pub struct Database {
  pub auctions: HashMap<data::AuctionId, data::Auction>,
  pub bids: HashMap<data::BidId, data::Bid>,
  pub comments: HashMap<data::CommentId, data::Comment>,
  pub shows: HashMap<data::ShowId, data::Show>,
  pub products: HashMap<data::ProductId, data::Product>,
  pub users: HashMap<data::UserId, data::User>,
}

pub fn db() -> DatabaseGuard {
  DATABASE
    .get_or_init(|| Mutex::new(Database::default()))
    .lock()
    .map_err(DatabaseError::Lock)
    .unwrap()
}
