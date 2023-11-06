pub mod graphql;
pub mod index;

use bits_graphql::Schema;
use poem::get;
use poem::Route;

pub fn router(schema: &Schema) -> Route {
  Route::new()
    .nest("/graphql", graphql::router(schema.clone()))
    .at("/", get(index::index_handler))
    .at("/:name", get(index::show_handler))
}
