use crate::mutations::bid_mutation;
use crate::mutations::comment_mutation;
use crate::mutations::create_auction_mutation;
use crate::mutations::create_product_mutation;
use crate::mutations::create_show_mutation;
use crate::mutations::start_mutation;
use async_graphql::dynamic::SchemaBuilder;
use bits_core::data::auction;
use bits_core::data::bid;
use bits_core::data::comment;
use bits_core::data::person;
use bits_core::data::product;
use bits_core::data::show;
use bits_core::sea_orm;
use bits_core::sea_orm::DatabaseConnection;
use bits_core::seaography;
use bits_core::seaography::register_entities;
use bits_core::seaography::Builder;
use bits_core::seaography::BuilderContext;

/// The GraphQL schema.
pub type Schema = async_graphql::dynamic::Schema;

/// Build the GraphQL schema.
pub fn schema(
  context: &'static BuilderContext,
  connection: DatabaseConnection,
) -> SchemaBuilder {
  let builder = Builder::new(context, connection.clone());
  let builder = register_entities(builder);
  let builder = register_mutations(builder);

  // TODO: limit depth & complexity.
  builder.schema_builder().data(connection)
}

fn register_entities(mut builder: Builder) -> Builder {
  register_entities!(builder, [auction, bid, comment, person, product, show,]);
  builder
}

#[allow(clippy::let_and_return)]
fn register_mutations(mut builder: Builder) -> Builder {
  builder.mutations = Vec::new();
  let builder = bid_mutation::register(builder);
  let builder = comment_mutation::register(builder);
  let builder = create_auction_mutation::register(builder);
  let builder = create_product_mutation::register(builder);
  let builder = create_show_mutation::register(builder);
  let builder = start_mutation::register(builder);
  builder
}
