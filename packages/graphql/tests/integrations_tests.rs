mod utils;

use crate::utils::execute;
use crate::utils::Context;
use crate::utils::TestToken;
use graphql_client::GraphQLQuery;
#[cfg(test)]
use insta::assert_json_snapshot;
use test_context::test_context;
use tokio::test;

#[rustfmt::skip]
#[derive(graphql_client::GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct CreateShowMutation;

#[test_context(Context)]
#[test]
async fn test_create_show_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    CreateShowMutation::build_query(create_show_mutation::Variables {}),
    Some(TestToken::seller_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.createShow.show.id" => "[uuid]" });
}

#[rustfmt::skip]
#[derive(GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct CreateProductMutation;

#[test_context(Context)]
#[test]
async fn test_create_product_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    CreateProductMutation::build_query(create_product_mutation::Variables {}),
    Some(TestToken::seller_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.createProduct.product.id" => "[uuid]" });
}

#[rustfmt::skip]
#[derive(GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct CreateAuctionMutation;

#[test_context(Context)]
#[test]
async fn test_create_auction_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    CreateAuctionMutation::build_query(create_auction_mutation::Variables {}),
    Some(TestToken::seller_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.createAuction.auction.id" => "[uuid]" });
}

#[rustfmt::skip]
#[derive(GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct StartMutation;

#[test_context(Context)]
#[test]
async fn test_start_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    StartMutation::build_query(start_mutation::Variables {}),
    Some(TestToken::seller_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.start.auction.id" => "[uuid]" });
}

#[rustfmt::skip]
#[derive(GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct CommentMutation;

#[test_context(Context)]
#[test]
async fn test_comment_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    CommentMutation::build_query(comment_mutation::Variables {}),
    Some(TestToken::buyer_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.comment.comment.id" => "[uuid]" });
}

#[rustfmt::skip]
#[derive(GraphQLQuery)]
#[graphql(schema_path = "../../docs/schema.gql", query_path = "tests/operations.graphql")]
pub struct BidMutation;

#[test_context(Context)]
#[test]
async fn test_bid_mutation(ctx: &mut Context) {
  let response = execute(
    ctx,
    BidMutation::build_query(bid_mutation::Variables {}),
    Some(TestToken::buyer_token()),
  )
  .await
  .unwrap();

  assert_json_snapshot!(response, { ".data.bid.bid.id" => "[uuid]" });
}
