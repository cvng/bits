use crate::entities::auction;
use crate::entities::bid;
use crate::entities::comment;
use crate::entities::person;
use crate::entities::product;
use crate::entities::show;
use async_graphql::dataloader::DataLoader;
use async_graphql::dynamic::SchemaBuilder;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use seaography::Builder;
use seaography::BuilderContext;

lazy_static! {
  static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub type Schema = async_graphql::dynamic::Schema;

pub struct OrmDataLoader {
  pub db: DatabaseConnection,
}

/// Build the GraphQL schema.
pub fn schema(connection: DatabaseConnection) -> SchemaBuilder {
  let dataloader = DataLoader::new(
    OrmDataLoader {
      db: connection.clone(),
    },
    tokio::spawn,
  );

  let mut builder = Builder::new(&CONTEXT, connection.clone());

  seaography::register_entities!(
    builder,
    [auction, bid, comment, person, product, show,]
  );

  // TODO: register_mutations(builder, connection, dataloader);

  builder
    .schema_builder()
    // TODO(security): .limit_depth(5).limit_complexity(5) @cvng
    .data(connection)
    .data(dataloader)
}
