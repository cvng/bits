#[derive(Debug)]
pub enum Error {
    InternalError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
