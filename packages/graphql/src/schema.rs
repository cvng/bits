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

pub struct OrmDataloader {
  pub db: DatabaseConnection,
}

pub fn schema(
  connection: DatabaseConnection,
  dataloader: DataLoader<OrmDataloader>,
) -> SchemaBuilder {
  let mut builder = Builder::new(&CONTEXT, connection.clone());

  seaography::register_entities!(
    builder,
    [auction, bid, comment, person, product, show,]
  );

  builder.schema_builder().data(connection).data(dataloader)
}
