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
  let response = common::execute(
    TestToken::bidder(),
    BidMutation::build_query(bid_mutation::Variables {}),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" });
}

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "tests/operations.graphql"
)]
pub struct StartMutation;

#[test]
async fn test_start_mutation() {
  let response = common::execute(
    TestToken::seller(),
    StartMutation::build_query(start_mutation::Variables {}),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.start.show.id" => "[uuid]" });
}
