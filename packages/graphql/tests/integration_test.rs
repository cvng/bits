mod setup;

use crate::setup::try_into_request;
use graphql_client::GraphQLQuery;
use insta::assert_json_snapshot;
use tokio::test;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "../../docs/schema.gql",
  query_path = "tests/mutations.graphql"
)]
pub struct BidMutation;

#[test]
async fn test_bid_mutation() {
  let (schema, client, token) = setup::setup().await;

  let request =
    try_into_request(BidMutation::build_query(bid_mutation::Variables {
      input: bid_mutation::BidInput {
        auction_id: "00000000-0000-0000-0000-000000000000".parse().unwrap(),
        bidder_id: "00000000-2000-0000-0000-000000000000".parse().unwrap(),
        amount: 1000,
      },
    }))
    .unwrap()
    .data(client)
    .data(token);

  let response = schema.execute(request).await.into_result().unwrap();

  assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" }, @r###"
  {
    "data": {
      "bid": {
        "bid": {
          "id": "[uuid]",
          "amount": "999"
        }
      }
    }
  }
  "###);
}
