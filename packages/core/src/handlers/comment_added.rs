use crate::database;
use crate::error::Result;
use bits_data::Comment;
use bits_data::CommentAdded;

pub fn comment_added(event: CommentAdded) -> Result<()> {
  let comment = Comment {
    id: event.id,
    user_id: event.user_id,
    show_id: event.show_id,
    text: event.text,
  };

  database::db().comments.insert(comment.id, comment);

  Ok(())
}
