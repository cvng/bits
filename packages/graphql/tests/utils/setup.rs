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

const BUYER_TOKEN: &str = include_str!("token_buyer.txt");
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
  pub fn buyer_token() -> Self {
    Self(Token(BUYER_TOKEN.to_string()))
  }

  pub fn seller_token() -> Self {
    Self(Token(SELLER_TOKEN.to_string()))
  }
}

pub async fn setup() -> Context {
  dotenv::dotenv().ok();

  let database_url = env::var("DATABASE_URL").unwrap();
  let connection = Database::connect(&database_url).await.unwrap();
  let client = Client::default().connection(connection.clone());
  let schema = bits_graphql::schema(&BUILDER, client.clone())
    .finish()
    .unwrap();

  Context { client, schema }
}

pub async fn execute<V>(
  ctx: &mut Context,
  query_body: QueryBody<V>,
  test_token: Option<TestToken>,
) -> Result<Response, Vec<ServerError>>
where
  V: Serialize,
{
  let mut request = try_into_request(query_body).unwrap();

  if let Some(test_token) = test_token {
    request = request.data(ctx.client.clone().token(test_token.0));
  }

  ctx.schema.execute(request).await.into_result()
}
