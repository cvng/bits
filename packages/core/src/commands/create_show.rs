use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Show;
use bits_data::ShowCreated;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use thiserror::Error;

#[derive(InputObject)]
pub struct CreateShowInput {
  pub creator_id: UserId,
  pub name: Text,
}

#[derive(SimpleObject)]
pub struct CreateShowPayload {
  pub show: Show,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ShowId),
}

pub async fn create_show(
  input: CreateShowInput,
) -> Result<CreateShowPayload, Error> {
  let show = Show {
    id: ShowId::new(),
    creator_id: input.creator_id,
    name: input.name,
    ready_at: None,
    started_at: None,
  };

  dispatch::dispatch(vec![ShowCreated { show }.into()]).ok();

  Ok(CreateShowPayload {
    show: database::db()
      .shows
      .get(&show.id)
      .cloned()
      .ok_or(Error::NotFound(show.id))?,
  })
}
