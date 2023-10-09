use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::ShowStarted;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct StartShowInput {
  pub id: ShowId,
}

#[derive(SimpleObject)]
pub struct StartShowPayload {
  pub show: Show,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ShowId),
  #[error("already started: {0}")]
  AlreadyStarted(ShowId),
  #[error("missing products: {0}")]
  MissingProducts(ShowId),
}

pub async fn start_show(
  input: StartShowInput,
) -> Result<StartShowPayload, Error> {
  let show = database::db()
    .shows
    .get(&input.id)
    .cloned()
    .ok_or(Error::NotFound(input.id))?;

  // Check that the show hasn't already started.
  if show.started_at.is_some() {
    return Err(Error::AlreadyStarted(show.id));
  }

  let show_products = database::db()
    .show_products
    .values()
    .filter(|show_product| show_product.show_id == show.id)
    .cloned()
    .collect::<Vec<_>>();

  // Check that there are products for this show.
  if show_products.is_empty() {
    return Err(Error::MissingProducts(show.id));
  }

  dispatch::dispatch(vec![ShowStarted {
    id: show.id,
    started_at: Utc::now(),
  }
  .into()])
  .ok();

  Ok(StartShowPayload {
    show: database::db()
      .shows
      .get(&input.id)
      .cloned()
      .ok_or(Error::NotFound(input.id))?,
  })
}
