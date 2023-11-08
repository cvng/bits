pub mod graphql;
pub mod index;

use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn router(state: AppState) -> Router {
  Router::new()
    .nest_service("/graphql", graphql::router(state.clone()))
    .route("/", get(index::IndexTemplate::handler))
    .route("/:name", get(index::ShowTemplate::handler))
    .with_state(state)
}
