use crate::decoder;
use bits_data::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(Uuid),
  #[error("jsonwebtoken error")]
  Decoder(#[from] decoder::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
