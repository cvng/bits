use crate::query::QueryRoot;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::SchemaBuilder;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
}
