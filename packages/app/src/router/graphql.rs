use crate::AppState;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use async_graphql_axum::GraphQLSubscription;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use bits_graphql::core::Token;
use bits_graphql::GraphiQLSource;
use http::header;
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
  state: State<AppState>,
  headers: HeaderMap,
  request: GraphQLRequest,
) -> GraphQLResponse {
  let mut request = request.into_inner();

  if let Some(token) = get_token_from_headers(headers) {
    request = request.data(state.client.clone().token(token));
  }

  state.schema.execute(request).await.into()
}

fn get_token_from_headers(headers: HeaderMap) -> Option<Token> {
  headers.get(header::AUTHORIZATION).and_then(|value| {
    value
      .to_str()
      .map(|s| s.replace("Bearer ", ""))
      .map(Token)
      .ok()
  })
}

pub fn router(state: AppState) -> Router {
  Router::new()
    .route("/", post(graphql_handler))
    .route("/playground", get(graphiql_handler))
    .route_service("/ws", GraphQLSubscription::new(state.schema.clone()))
    .with_state(state)
}
