mod setup;

use graphql_client::GraphQLQuery;
use tokio::test;
use bits_graphql::try_into_request;

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

  insta::assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" }, @r###"
  {
    "data": {
      "bid": {
        "bid": {
          "id": "[uuid]",
          "auctionId": "00000000-0000-0000-0000-000000000000",
          "bidderId": "00000000-2000-0000-0000-000000000000",
          "amount": "1000"
        }
      }
    }
  }
  "###);
}
