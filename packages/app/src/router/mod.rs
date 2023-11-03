pub mod graphql;

use axum::Router;
use bits_graphql::Schema;

pub fn router(schema: Schema) -> Router {
  Router::new().nest_service("/graphql", graphql::router(schema.to_owned()))
}
