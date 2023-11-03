use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use async_graphql_axum::GraphQLSubscription;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use bits_graphql::Schema;
use bits_graphql::Token;
use http::HeaderMap;

async fn graphiql_handler() -> impl IntoResponse {
  Html(
    GraphiQLSource::build()
      .endpoint("/graphql")
      .subscription_endpoint("/graphql/ws")
      .finish(),
  )
}

async fn graphql_handler(
  schema: State<Schema>,
  headers: HeaderMap,
  request: GraphQLRequest,
) -> GraphQLResponse {
  let mut request = request.into_inner();

  if let Some(token) = get_token_from_headers(&headers) {
    request = request.data(token);
  }

  schema.execute(request).await.into()
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
  headers.get("Authorization").and_then(|value| {
    value
      .to_str()
      .map(|s| s.replace("Bearer ", ""))
      .map(Token)
      .ok()
  })
}

pub fn router(schema: Schema) -> Router {
  Router::new()
    .route("/graphql", get(graphql_handler))
    .route("/graphql/playground", get(graphiql_handler))
    .route_service("/graphql/ws", GraphQLSubscription::new(schema.to_owned()))
    .with_state(schema)
}
