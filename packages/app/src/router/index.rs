use crate::filters;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use bits_graphql::Schema;
use graphql_client::GraphQLQuery;
use serde_json::from_value;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "templates/index.graphql"
)]
struct IndexQuery;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
  data: index_query::ResponseData,
}

pub async fn handler(schema: State<Schema>) -> impl IntoResponse {
  IndexTemplate {
    data: schema
      .execute(IndexQuery::build_query(index_query::Variables {}).query)
      .await
      .data
      .into_json()
      .map(from_value::<index_query::ResponseData>)
      .unwrap()
      .unwrap(),
  }
}
