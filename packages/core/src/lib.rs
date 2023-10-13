#[macro_use]
#[cfg(test)]
extern crate insta;

pub mod commands;
mod database;
mod dispatch;
mod error;
mod handlers;

pub use bits_data::*;
pub use commands::*;
pub use database::db;
pub use error::*;
