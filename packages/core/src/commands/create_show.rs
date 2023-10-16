use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Event;
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
pub struct CreateShowResult {
  pub show: Show,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("not found: {0}")]
  NotFound(ShowId),
}

pub async fn create_show(
  input: CreateShowInput,
) -> Result<CreateShowResult, Error> {
  let show = Show {
    id: ShowId::new(),
    creator_id: input.creator_id,
    name: input.name,
    started_at: None,
  };

  dispatcher::dispatch(vec![Event::ShowCreated(ShowCreated { show })]).ok();

  Ok(CreateShowResult { show })
}
