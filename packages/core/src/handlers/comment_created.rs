use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::CommentCreated;

pub fn comment_created(event: CommentCreated) -> Result<()> {
  database::db()
    .comments
    .insert(event.comment.id, event.comment.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.comment.id))
}
