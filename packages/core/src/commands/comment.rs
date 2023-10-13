use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Comment;
use bits_data::CommentCreated;
use bits_data::CommentId;
use bits_data::Event;
#[cfg(test)]
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use serde::Serialize;
use thiserror::Error;

#[derive(InputObject)]
pub struct CommentInput {
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

#[derive(SimpleObject, Serialize)]
pub struct CommentPayload {
  pub comment: Comment,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

pub fn comment(input: CommentInput) -> Result<CommentPayload, Error> {
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

#[test]
fn test_comment() {
  database::db().shows.insert(
    "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    Show {
      id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      creator_id: UserId::new(),
      name: Text::new("name"),
      started_at: None,
    },
  );

  let payload = comment(CommentInput {
    user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    text: Text::new("text"),
  })
  .unwrap();

  assert_json_snapshot!(payload, {".comment.id" => "[uuid]"}, @r###"
  {
    "comment": {
      "id": "[uuid]",
      "user_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
      "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
      "text": "text"
    }
  }
  "###);
}
