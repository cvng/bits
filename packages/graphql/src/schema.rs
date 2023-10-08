use crate::mutation::MutationRoot;
use crate::query::QueryRoot;
use async_graphql::EmptySubscription;
use async_graphql::SchemaBuilder;

pub type Schema =
  async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
  Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}
