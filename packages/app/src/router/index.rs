use crate::filters;
use crate::utils::into_response;
use askama::Template;
use bits_graphql::try_into_request;
use bits_graphql::Schema;
use graphql_client::GraphQLQuery;
use poem::handler;
use poem::web::Data;
use poem::web::Path;
use poem::IntoResponse;
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
  fn into_response(self) -> impl IntoResponse {
    into_response(&self)
  }
}

#[handler]
pub async fn index_handler(schema: Data<&Schema>) -> impl IntoResponse {
  IndexTemplate {
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
  .into_response()
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
  fn into_response(self) -> impl IntoResponse {
    into_response(&self)
  }
}

#[handler]
pub async fn show_handler(
  schema: Data<&Schema>,
  name: Path<String>,
) -> impl IntoResponse {
  ShowTemplate {
    data: schema
      .execute(
        try_into_request(ShowQuery::build_query(show_query::Variables {
          name: name.0,
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
  .into_response()
}
