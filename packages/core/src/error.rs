use bits_data::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(Uuid),
  #[error("jsonwebtoken error")]
  Jwt(#[from] jsonwebtoken::errors::Error),
  #[error("uuid error")]
  Uuid(#[from] uuid::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
