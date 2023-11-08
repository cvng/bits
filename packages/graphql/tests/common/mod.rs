use async_graphql::Response;
use async_graphql::ServerError;
use bits_core::sea_orm::prelude::async_trait::async_trait;
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
use test_context::AsyncTestContext;

const BIDDER_TOKEN: &str = include_str!("token_bidder.txt");
const SELLER_TOKEN: &str = include_str!("token_seller.txt");

lazy_static! {
  static ref BUILDER: seaography::BuilderContext = BuilderContext::custom();
}

pub struct Context {
  schema: Schema,
  client: Client,
}

#[async_trait]
impl AsyncTestContext for Context {
  async fn setup() -> Self {
    setup().await
  }
}

pub struct TestToken(pub Token);

impl TestToken {
  pub fn bidder_token() -> Self {
    Self(Token(BIDDER_TOKEN.to_string()))
  }

  pub fn seller_token() -> Self {
    Self(Token(SELLER_TOKEN.to_string()))
  }
}

pub async fn setup() -> Context {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL").unwrap();
  let connection = Database::connect(&database_url).await.unwrap();

  let client = Client::default().connection(&connection);
  let schema = bits_graphql::schema(&BUILDER, &client).finish().unwrap();

  Context { schema, client }
}

pub async fn execute<V>(
  ctx: &mut Context,
  test_token: TestToken,
  query_body: QueryBody<V>,
) -> Result<Response, Vec<ServerError>>
where
  V: Serialize,
{
  let client = ctx.client.clone();
  let schema = ctx.schema.clone();

  let request = try_into_request(query_body)
    .unwrap()
    .data(client)
    .data(test_token.0);

  schema.execute(request).await.into_result()
}
