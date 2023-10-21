// https://github.com/launchbadge/sqlx/tree/main/examples/postgres/listen

use serde_json::Value;
use sqlx::postgres::PgListener;
use sqlx::postgres::PgNotification;

const NOTIFICATION_CHANNEL: &str = "cqrs.event";

#[derive(Debug, Deserialize)]
pub struct Event {
  pub id: i32,
  pub created: String,
  pub r#type: String,
  pub data: Value,
}

pub async fn listen(
  database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let pool = sqlx::PgPool::connect(database_url).await?;
  let mut listener = PgListener::connect_with(&pool).await?;

  listener.listen(NOTIFICATION_CHANNEL).await?;

  loop {
    handler(listener.recv().await?)?;
  }
}

fn handler(
  notification: PgNotification,
) -> Result<(), Box<dyn std::error::Error>> {
  let event = serde_json::from_str::<Event>(notification.payload())?;

  println!("{event:#?}");

  Ok(())
}
