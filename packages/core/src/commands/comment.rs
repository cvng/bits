use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Comment;
use bits_data::CommentCreated;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use thiserror::Error;

#[derive(InputObject)]
pub struct CommentInput {
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

#[derive(SimpleObject)]
pub struct CommentPayload {
  pub comment: Comment,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

pub async fn comment(input: CommentInput) -> Result<CommentPayload, Error> {
  database::db()
    .shows
    .get(&input.show_id)
    .cloned()
    .ok_or(Error::ShowNotFound(input.show_id))?;

  let comment = Comment {
    id: CommentId::new(),
    user_id: input.user_id,
    show_id: input.show_id,
    text: input.text,
  };

  dispatch::dispatch(vec![Event::CommentCreated(CommentCreated { comment })])
    .ok();

  Ok(CommentPayload { comment })
}
