use crate::database;
use crate::dispatch;
use crate::Error;
use crate::Result;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Show;
use bits_data::ShowCreated;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;

#[derive(InputObject)]
pub struct CreateShowInput {
  pub creator_id: UserId,
  pub name: Text,
}

#[derive(SimpleObject)]
pub struct CreateShowPayload {
  pub show: Show,
}

pub async fn create_show(input: CreateShowInput) -> Result<CreateShowPayload> {
  let show = Show {
    id: ShowId::new(),
    creator_id: input.creator_id,
    name: input.name,
    started_at: None,
  };

  dispatch::dispatch(vec![ShowCreated { show }.into()])?;

  Ok(CreateShowPayload {
    show: database::db()
      .shows
      .get(&show.id)
      .cloned()
      .ok_or_else(|| Error::NotFound(show.id.into()))?,
  })
}
