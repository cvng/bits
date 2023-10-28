use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use async_graphql_axum::GraphQLSubscription;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use bits_graphql::Schema;
use bits_graphql::Token;
use http::HeaderMap;
use tower_http::cors::CorsLayer;

pub type Server<I, S> = axum::Server<I, S>;

async fn graphql_playground() -> impl IntoResponse {
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

pub fn app(schema: Schema) -> Router {
  let cors = CorsLayer::permissive();

  let graphql = post(graphql_handler); // TODO: GraphQL::new(schema.to_owned());
  let graphql_subscription = GraphQLSubscription::new(schema.to_owned());

  Router::new()
    .route("/graphql", graphql)
    .route_service("/graphql/ws", graphql_subscription)
    .route("/graphql/playground", get(graphql_playground))
    .with_state(schema)
    .layer(cors)
}
