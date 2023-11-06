mod filters;
mod router;

use axum::Server;
use bits_graphql::core;
use bits_graphql::core::Client;
use bits_graphql::core::Database;
use rust_i18n::i18n;
use std::env;
use thiserror::Error;
use tokio::main;
use tracing::Level;
use tracing::info;

#[derive(Debug, Error)]
enum Error {
  #[error("env variable `{0}` not set")]
  Config(String),
  #[error("failed to connect database")]
  Database(#[from] core::DbErr),
  #[error("failed to build GraphQL schema")]
  Schema(#[from] bits_graphql::SchemaError),
  #[error("failed to start web server")]
  Server(#[from] hyper::Error),
}

i18n!("locales", fallback = "en");

#[main]
async fn main() {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt().with_max_level(Level::INFO).init();

  let addr = env::var("ADDR")
    .unwrap_or("0.0.0.0:8000".to_string())
    .parse()
    .map_err(|_| Error::Config("ADDR".to_string()))
    .unwrap();

  let database_url = env::var("DATABASE_URL")
    .map_err(|_| Error::Config("DATABASE_URL".to_string()))
    .unwrap();

  let connection = Database::connect(&database_url)
    .await
    .map_err(Error::Database)
    .unwrap();

  let client = Client::default().connection(&connection);

  let schema = bits_graphql::schema(&client)
    .finish()
    .map_err(Error::Schema)
    .unwrap();

  let router = router::router(&schema);

  info!(addr = %addr, "listening");

  Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .map_err(Error::Server)
    .unwrap()
}
