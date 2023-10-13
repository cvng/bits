use crate::database;
use crate::error::Result;
use bits_data::CommentCreated;

pub fn comment_created(event: CommentCreated) -> Result<()> {
  let comment = event.comment;

  database::db().comments.insert(comment.id, comment);

  Ok(())
}
