use crate::database;
use crate::error::Result;
use bits_data::Comment;
use bits_data::CommentAdded;
use bits_data::CommentId;

pub fn comment_added(event: CommentAdded) -> Result<()> {
  let comment = Comment {
    id: CommentId::new(),
    user_id: event.user_id,
    show_id: event.show_id,
  };

  database::db().comments.insert(comment.id, comment);

  Ok(())
}
