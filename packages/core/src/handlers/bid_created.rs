use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::BidCreated;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseBackend;
use sea_orm::Statement;
use sea_orm::TransactionTrait;

pub async fn bid_created(client: &Client, event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
    .ok();

  let txn = client.connection.begin().await.unwrap();

  txn
    .execute(Statement::from_string(
      DatabaseBackend::Postgres,
      "select auth.login('bidder', '00000000-2000-0000-0000-000000000000');",
    ))
    .await
    .unwrap();

  txn
    .execute(Statement::from_sql_and_values(
      DatabaseBackend::Postgres,
      r#"insert into cqrs.event (type, data) values ($1::cqrs.event_type, $2::jsonb)"#,
      ["bid_created".into(), serde_json::to_value(&event.bid).unwrap().into()],
    ))
    .await
    .unwrap();

  txn.commit().await.unwrap();

  Ok(())
}
