use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found")]
  NotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
