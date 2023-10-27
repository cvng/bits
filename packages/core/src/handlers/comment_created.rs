use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Context;
use bits_data::CommentCreated;

pub async fn comment_created(
  _ctx: &Context,
  event: CommentCreated,
) -> Result<()> {
  database::db()
    .comments
    .insert(event.comment.id, event.comment.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.comment.id))
}
