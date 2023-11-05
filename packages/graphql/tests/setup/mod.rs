use bits_core::Client;
use bits_core::Database;
use bits_core::Token;
use bits_graphql::Schema;
use std::env;

pub type Setup = (Schema, Client, Token);

pub async fn setup() -> Setup {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL environment variable not set");

  let connection = Database::connect(&database_url)
    .await
    .expect("Fail to initialize database connection");

  let token_str =
    env::var("TEST_TOKEN").expect("TEST_TOKEN environment variable not set");

  let token = Token(token_str);

  let client = Client::default().connection(&connection);

  let schema = bits_graphql::schema(&client)
    .finish()
    .expect("Fail to initialize GraphQL schema");

  let client = Client::default().connection(&connection);

  (schema, client, token)
}
