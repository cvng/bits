use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use async_graphql_axum::GraphQLSubscription;
use axum::response;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use bits_graphql::Schema;
use http::Method;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub type Server<I, S> = axum::Server<I, S>;

async fn graphiql() -> impl IntoResponse {
  response::Html(
    GraphiQLSource::build()
      .endpoint("/graphql")
      .subscription_endpoint("/graphql/ws")
      .finish(),
  )
}

pub fn app(schema: Schema) -> Router {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any)
    .allow_origin(Any);

  Router::new()
    .route(
      "/graphql",
      get(graphiql).post_service(GraphQL::new(schema.to_owned())),
    )
    .route_service("/graphql/ws", GraphQLSubscription::new(schema))
    .layer(cors)
}
