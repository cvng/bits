// https://github.com/launchbadge/sqlx/tree/main/examples/postgres/listen

use sqlx::postgres::PgListener;

const NOTIFICATION_CHANNEL: &str = "cqrs.event";

pub async fn listen(
  database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let pool = sqlx::PgPool::connect(database_url).await?;

  let mut listener = PgListener::connect_with(&pool).await?;

  listener.listen(NOTIFICATION_CHANNEL).await?;

  loop {
    let notification = listener.recv().await?;
    println!("[from recv]: {notification:?}");
  }
}
