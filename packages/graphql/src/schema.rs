use crate::mutation::MutationBuilder;
use async_graphql::dynamic::SchemaBuilder;
use bits_core::entities::auction;
use bits_core::entities::bid;
use bits_core::entities::comment;
use bits_core::entities::person;
use bits_core::entities::product;
use bits_core::entities::show;
use bits_core::sea_orm;
use bits_core::Context;
use bits_core::DatabaseConnection;
use lazy_static::lazy_static;
use seaography::register_entities;
use seaography::Builder;
use seaography::BuilderContext;

lazy_static! {
  static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub type Schema = async_graphql::dynamic::Schema;

/// Build the GraphQL schema. TODO: limit depth & complexity
pub fn schema(connection: DatabaseConnection) -> SchemaBuilder {
  let context = Context::new(connection.clone());

  let builder = Builder::new(&CONTEXT, connection.clone());
  let builder = register_entities(builder);
  let builder = register_mutations(builder);

  builder.schema_builder().data(connection).data(context)
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
