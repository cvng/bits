#[macro_use]
extern crate thiserror;

mod server;

use async_graphql::dynamic::SchemaError;
use bits_graphql::Client;
use bits_graphql::Database;
use bits_graphql::DbErr;
use server::Server;
use std::env;
use tokio::main;
use tracing::Level;

#[derive(Debug, Error)]
enum Error {
  #[error("env variable `{0}` not set")]
  Config(String),
  #[error("failed to connect database")]
  Database(#[from] DbErr),
  #[error("failed to build GraphQL schema")]
  Schema(#[from] SchemaError),
  #[error("failed to start web server")]
  Server(#[from] hyper::Error),
}

#[main]
async fn main() {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt().with_max_level(Level::INFO).init();

  let addr = "0.0.0.0:8000".parse().unwrap();

  let database_url = env::var("DATABASE_URL")
    .map_err(|_| Error::Config("DATABASE_URL".to_string()))
    .unwrap();

  let connection = Database::connect(&database_url)
    .await
    .map_err(Error::Database)
    .unwrap();

  let client = Client::default().connection(connection.clone());

  let schema = bits_graphql::schema(client)
    .finish()
    .map_err(Error::Schema)
    .unwrap();

  let router = server::app(schema);

  println!("GraphiQL IDE: http://{addr}/graphql");

  Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .map_err(Error::Server)
    .unwrap()
}
