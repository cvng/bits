pub mod graphql;
pub mod index;

use axum::routing::get;
use axum::Router;
use bits_graphql::Schema;

pub fn router(schema: &Schema) -> Router {
  Router::new()
    .nest_service("/graphql", graphql::router(schema.clone()))
    .route("/", get(index::index_handler))
    .route("/:name", get(index::show_handler))
    .with_state(schema.clone())
}
