mod common;

use crate::common::TestToken;
use graphql_client::GraphQLQuery;
#[cfg(test)]
use insta::assert_json_snapshot;
use tokio::test;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "tests/operations.graphql"
)]
pub struct BidMutation;

#[test]
async fn test_bid_mutation() {
  let (schema, client) = common::setup().await;
  let token = TestToken::bidder();

  let response = common::execute(
    schema,
    client,
    token,
    BidMutation::build_query(bid_mutation::Variables {}),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" });
}
