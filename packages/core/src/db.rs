use bits_data::Show;
use bits_data::ShowId;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
use std::sync::PoisonError;
use thiserror::Error;

static DATABASE: OnceLock<Mutex<Database>> = OnceLock::new();

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database lock panicked")]
    Lock(#[from] PoisonError<MutexGuard<'static, Database>>),
}

#[derive(Default)]
pub struct Database {
    pub shows: HashMap<ShowId, Show>,
}

pub fn db<'a>() -> MutexGuard<'a, Database> {
    DATABASE
        .get_or_init(|| Mutex::new(Database::default()))
        .lock()
        .map_err(DatabaseError::Lock)
        .unwrap()
}
