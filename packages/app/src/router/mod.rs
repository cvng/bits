pub mod graphql;
pub mod index;

use axum::routing::get;
use axum::Router;
use bits_graphql::Schema;

pub fn router(schema: &Schema) -> Router {
  Router::new()
    .nest_service("/graphql", graphql::router(schema.clone()))
    .route("/", get(index::IndexTemplate::handler))
    .route("/:name", get(index::ShowTemplate::handler))
    .with_state(schema.clone())
}
