use crate::config::BuilderContext;
use crate::mutations::bid_mutation;
use crate::mutations::comment_mutation;
use crate::mutations::create_auction_mutation;
use crate::mutations::create_product_mutation;
use crate::mutations::create_show_mutation;
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

#[allow(clippy::let_and_return)]
fn register_mutations(mut builder: Builder) -> Builder {
  builder.mutations = Vec::new();
  let builder = bid_mutation::register(builder);
  let builder = comment_mutation::register(builder);
  let builder = create_auction_mutation::register(builder);
  let builder = create_product_mutation::register(builder);
  let builder = create_show_mutation::register(builder);
  builder
}
