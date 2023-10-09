use thiserror::Error;
use bits_data::Uuid;

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(Uuid),
  #[error("already started: {0}")]
  AlreadyStarted(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
