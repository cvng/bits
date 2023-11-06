use async_graphql::Response;
use async_graphql::ServerError;
use bits_core::seaography;
use bits_core::Client;
use bits_core::Database;
use bits_core::Token;
use bits_graphql::try_into_request;
use bits_graphql::BuilderContext;
use bits_graphql::Schema;
use graphql_client::QueryBody;
use lazy_static::lazy_static;
use serde::Serialize;
use std::env;

const BIDDER_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIwMDAwMDAwMC0yMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAiLCJuYW1lIjoiSm9obiBEb2UiLCJpYXQiOjE1MTYyMzkwMjJ9.MQf38zuzvH0ZB0zk7QbvzIH_b7jkiP92Jo39JTKy2PY";

lazy_static! {
  static ref CONTEXT: seaography::BuilderContext = BuilderContext::custom();
}

pub type Setup = (Schema, Client);

pub struct TestToken(pub Token);

impl TestToken {
  pub fn bidder() -> Self {
    Self(Token(BIDDER_TOKEN.to_string()))
  }
}

pub async fn setup() -> Setup {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL environment variable not set");

  let connection = Database::connect(&database_url)
    .await
    .expect("Fail to initialize database connection");

  let client = Client::default().connection(&connection);

  let schema = bits_graphql::schema(&CONTEXT, &client)
    .finish()
    .expect("Fail to initialize GraphQL schema");

  let client = Client::default().connection(&connection);

  (schema, client)
}

pub async fn execute<V>(
  test_token: TestToken,
  query_body: QueryBody<V>,
) -> Result<Response, Vec<ServerError>>
where
  V: Serialize,
{
  let (schema, client) = setup().await;

  let request = try_into_request(query_body)
    .unwrap()
    .data(client)
    .data(test_token.0);

  schema.execute(request).await.into_result()
}
