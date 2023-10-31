use crate::config::BuilderContext;
use crate::mutation::MutationBuilder;
use async_graphql::dynamic::SchemaBuilder;
use bits_core::entities::auction;
use bits_core::entities::bid;
use bits_core::entities::comment;
use bits_core::entities::person;
use bits_core::entities::product;
use bits_core::entities::show;
use bits_core::sea_orm;
use bits_core::Client;
use lazy_static::lazy_static;
use seaography::register_entities;
use seaography::Builder;

lazy_static! {
  static ref CONTEXT: seaography::BuilderContext = BuilderContext::custom();
}

/// The GraphQL schema.
pub type Schema = async_graphql::dynamic::Schema;

/// Build the GraphQL schema.
pub fn schema(client: Client) -> SchemaBuilder {
  let builder = Builder::new(&CONTEXT, client.connection.clone());
  let builder = register_entities(builder);
  let builder = register_mutations(builder);

  builder
    .schema_builder()
    .data(client.connection.clone())
    .data(client) // TODO: limit depth & complexity.
}

fn register_entities(mut builder: Builder) -> Builder {
  register_entities!(builder, [auction, bid, comment, person, product, show,]);
  builder
}

fn register_mutations(mut builder: Builder) -> Builder {
  let mut mutation_builder = MutationBuilder::new();
  mutation_builder.register();

  builder.mutations = mutation_builder.mutations;
  builder.inputs.extend(mutation_builder.inputs);
  builder.outputs.extend(mutation_builder.outputs);
  builder
}
