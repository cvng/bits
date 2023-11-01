use crate::decoder::insecure_get_token_sub;
use crate::Client;
use bits_data::Event;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseBackend;
use sea_orm::DbErr;
use sea_orm::RuntimeErr;
use sea_orm::Statement;
use sea_orm::TransactionTrait;
use sqlx::error::DatabaseError;
use uuid::Uuid;

const AUTH_LOGIN_QUERY: &str = "select auth.login($1);";

const CQRS_EVENT_QUERY: &str =
  "insert into cqrs.event (type, data) values ($1::cqrs.event_type, $2::jsonb)";

#[derive(Debug, Error)]
pub enum ConstraintError {
  #[error("bid validity check failed")]
  BidValidityCheck,
}

#[derive(Debug, Error)]
pub enum DispatchError {
  #[error("database error: {0}")]
  Database(#[from] DbErr),
  #[error("serde error: {0}")]
  Serde(#[from] serde_json::Error),
  #[error("jwt error: {0}")]
  Jwt(#[from] jsonwebtoken::errors::Error),
  #[error("constraint error: {0}")]
  Constraint(#[from] ConstraintError),
}

pub async fn dispatch(
  client: &Client,
  events: Vec<Event>,
) -> Result<Vec<Event>, DispatchError> {
  let user_id = client
    .token
    .as_ref()
    .map(|token| insecure_get_token_sub::<Uuid>(&token.0))
    .transpose()?
    .flatten();

  let txn = client.connection.begin().await?;

  if let Some(user_id) = user_id {
    txn
      .execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        AUTH_LOGIN_QUERY,
        [user_id.into()],
      ))
      .await?;
  }

  for event in &events {
    let event = serde_json::to_value(event)?;
    let event_type = event.get("type").unwrap().as_str().unwrap();
    let event_payload = event.get("payload").unwrap().as_str().unwrap();

    let res = txn
      .execute(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        CQRS_EVENT_QUERY,
        [event_type.into(), event_payload.into()],
      ))
      .await;

    if let Err(DbErr::Exec(RuntimeErr::SqlxError(
      sqlx::error::Error::Database(e),
    ))) = res
    {
      match to_constraint_err(e) {
        Some(err) => return Err(DispatchError::Constraint(err)),
        None => unimplemented!(), // TODO: return Err(DispatchError::Database(res.err().unwrap())),
      }
    }
  }

  txn.commit().await?;

  Ok(events)
}

fn to_constraint_err(err: Box<dyn DatabaseError>) -> Option<ConstraintError> {
  match err.constraint() {
    Some("bid_validity_check") => Some(ConstraintError::BidValidityCheck),
    _ => None,
  }
}
