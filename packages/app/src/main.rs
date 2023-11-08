mod filters;
mod router;

use axum::Server;
use bits_graphql::core;
use bits_graphql::core::Client;
use bits_graphql::core::Database;
use bits_graphql::seaography;
use bits_graphql::BuilderContext;
use bits_graphql::Schema;
use lazy_static::lazy_static;
use rust_i18n::i18n;
use std::env;
use thiserror::Error;
use tokio::main;
use tracing::info;
use tracing::Level;

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

lazy_static! {
  static ref BUILDER: seaography::BuilderContext = BuilderContext::custom();
}

#[derive(Clone)]
pub struct AppState {
  pub client: Client,
  pub schema: Schema,
}

#[main]
async fn main() -> Result<(), Error> {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt().with_max_level(Level::INFO).init();

  let addr = env::var("ADDR")
    .unwrap_or("0.0.0.0:8000".to_string())
    .parse()
    .map_err(|_| Error::Config("ADDR".to_string()))?;

  let database_url = env::var("DATABASE_URL")
    .map_err(|_| Error::Config("DATABASE_URL".to_string()))?;

  let database = Database::connect(&database_url)
    .await
    .map_err(Error::Database)?;

  let client = Client::default().connection(&database);

  let schema = bits_graphql::schema(&BUILDER, &client)
    .finish()
    .map_err(Error::Schema)?;

  let shared_state = AppState { client, schema };

  let app = router::router(shared_state);

  info!(addr = %addr, "listening");

  Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .map_err(Error::Server)
}
