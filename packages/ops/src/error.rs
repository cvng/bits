#[derive(Debug)]
pub enum Error {
    InternalError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<bits_core::Error> for Error {
    fn from(_error: bits_core::Error) -> Self {
        Error::InternalError
    }
}

pub type Result<T> = std::result::Result<T, Error>;
