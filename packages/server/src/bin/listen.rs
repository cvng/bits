use std::env;

#[tokio::main]
pub async fn main() {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL environment variable not set");

  println!("Starting LISTEN loop.");

  bits_graphql::listen(&database_url)
    .await
    .expect("Fail to start LISTEN loop");
}
