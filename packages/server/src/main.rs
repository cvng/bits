mod server;

use async_graphql::dataloader::DataLoader;
use bits_graphql::OrmDataloader;
use sea_orm::Database;
use server::Server;
use std::env;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let addr = "0.0.0.0:8000".parse().unwrap();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL environment variable not set");

  let connection = Database::connect(database_url)
    .await
    .expect("Fail to initialize database connection");

  let dataloader: DataLoader<OrmDataloader> = DataLoader::new(
    OrmDataloader {
      db: connection.clone(),
    },
    tokio::spawn,
  );

  let schema = bits_graphql::schema(connection, dataloader)
    // TODO: .limit_depth(5).limit_complexity(5)
    .finish()
    .expect("Fail to initialize GraphQL schema");

  println!("GraphiQL IDE: http://{addr}/graphql");

  Server::bind(&addr)
    .serve(server::app(schema).into_make_service())
    .await
    .expect("Fail to start web server");
}
