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

  let client = Client::default().connection(connection.clone());

  let schema = bits_graphql::schema(client)
    .finish()
    .expect("Fail to initialize GraphQL schema");

  let client = Client::default().connection(connection);

  (schema, client, token)
}

pub fn try_into_request<V>(
  query_body: graphql_client::QueryBody<V>,
) -> Result<async_graphql::Request, serde_json::Error>
where
  V: serde::Serialize,
{
  let query = query_body.query;
  let operation_name = query_body.operation_name;
  let variables = serde_json::to_value(query_body.variables)?;
  let variables = async_graphql::Variables::from_json(variables);

  let request = async_graphql::Request::new(query)
    .operation_name(operation_name)
    .variables(variables);

  Ok(request)
}
