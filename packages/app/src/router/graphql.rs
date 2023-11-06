use async_graphql_poem::GraphQLRequest;
use async_graphql_poem::GraphQLResponse;
use async_graphql_poem::GraphQLSubscription;
use bits_graphql::core::Token;
use bits_graphql::GraphiQLSource;
use bits_graphql::Schema;
use http::header;
use http::HeaderMap;
use poem::get;
use poem::handler;
use poem::post;
use poem::web::Data;
use poem::web::Html;
use poem::IntoResponse;
use poem::Route;

#[handler]
async fn graphiql_handler() -> impl IntoResponse {
  Html(
    GraphiQLSource::build()
      .endpoint("/graphql")
      .subscription_endpoint("/graphql/ws")
      .finish(),
  )
}

#[handler]
async fn graphql_handler(
  schema: Data<&Schema>,
  headers: &HeaderMap,
  request: GraphQLRequest,
) -> GraphQLResponse {
  let mut request = request.0;

  if let Some(token) = get_token_from_headers(headers) {
    request = request.data(token);
  }

  schema.execute(request).await.into()
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
  headers.get(header::AUTHORIZATION).and_then(|value| {
    value
      .to_str()
      .map(|s| s.replace("Bearer ", ""))
      .map(Token)
      .ok()
  })
}

pub fn router(schema: Schema) -> Route {
  Route::new()
    .at("/", post(graphql_handler))
    .at("/playground", get(graphiql_handler))
    .at("/ws", GraphQLSubscription::new(schema.clone()))
}
