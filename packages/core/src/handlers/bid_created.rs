use crate::database;
use crate::decoder::insecure_get_token_sub;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::BidCreated;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseBackend;
use sea_orm::Statement;
use sea_orm::TransactionTrait;
use uuid::Uuid;

pub async fn bid_created(client: &Client, event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
    .ok();

  let sub =
    insecure_get_token_sub::<Uuid>(client.token.as_ref().unwrap().0.as_str())?
      .unwrap();

  let txn = client.connection.begin().await.unwrap();

  txn
    .execute(Statement::from_sql_and_values(
      DatabaseBackend::Postgres,
      "select auth.login($1);",
      [sub.into()],
    ))
    .await
    .unwrap();

  txn
    .execute(Statement::from_sql_and_values(
      DatabaseBackend::Postgres,
      "insert into cqrs.event (type, data) values ($1::cqrs.event_type, $2::jsonb)",
      ["bid_created".into(), serde_json::to_value(&event.bid).unwrap().into()]
    )).await.unwrap();

  txn.commit().await.unwrap();

  Ok(())
}
