use crate::filters;
use askama::Template;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use bits_graphql::try_into_request;
use bits_graphql::Schema;
use graphql_client::GraphQLQuery;
use serde_json::from_value;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "templates/pages/index.graphql"
)]
struct IndexQuery;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
  data: index_query::ResponseData,
}

impl IndexTemplate {
  pub async fn handler(schema: State<Schema>) -> impl IntoResponse {
    Self {
      data: schema
        .execute(
          try_into_request(IndexQuery::build_query(index_query::Variables {}))
            .unwrap(),
        )
        .await
        .data
        .into_json()
        .map(from_value::<index_query::ResponseData>)
        .unwrap()
        .unwrap(),
    }
  }
}

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "templates/pages/show.graphql"
)]
struct ShowQuery;

#[derive(Template)]
#[template(path = "pages/show.html")]
pub struct ShowTemplate {
  data: show_query::ResponseData,
}

impl ShowTemplate {
  pub async fn handler(
    schema: State<Schema>,
    Path(name): Path<String>,
  ) -> impl IntoResponse {
    Self {
      data: schema
        .execute(
          try_into_request(ShowQuery::build_query(show_query::Variables {
            name,
          }))
          .unwrap(),
        )
        .await
        .data
        .into_json()
        .map(from_value::<show_query::ResponseData>)
        .unwrap()
        .unwrap(),
    }
  }
}
