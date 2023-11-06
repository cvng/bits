use async_graphql::Request;
use async_graphql::Variables;
use graphql_client::QueryBody;
use serde::Serialize;
use serde_json::to_value;

pub fn try_into_request<V>(
  query_body: QueryBody<V>,
) -> Result<Request, serde_json::Error>
where
  V: Serialize,
{
  let request = Request::new(query_body.query)
    .operation_name(query_body.operation_name)
    .variables(Variables::from_json(to_value(query_body.variables)?));

  Ok(request)
}
