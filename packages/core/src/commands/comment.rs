use crate::database;
use crate::dispatch;
use crate::dispatch::Command;
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

#[derive(Clone, Copy, InputObject)]
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

pub struct CommentCommand {
  pub comment: Comment,
}

impl Command for CommentCommand {
  type Error = Error;
  type Input = CommentInput;
  type Payload = CommentPayload;

  fn new(input: Self::Input) -> Self {
    Self {
      comment: Comment {
        id: CommentId::new(),
        user_id: input.user_id,
        show_id: input.show_id,
        text: input.text,
      },
    }
  }

  fn run(&self) -> Result<CommentPayload, Error> {
    database::db()
      .shows
      .get(&self.comment.show_id)
      .cloned()
      .ok_or(Error::ShowNotFound(self.comment.show_id))?;

    dispatch::dispatch(self.handle()?).ok();

    Ok(CommentPayload {
      comment: self.comment,
    })
  }

  fn handle(&self) -> Result<Vec<Event>, Error> {
    let comment = self.comment;

    Ok(vec![Event::CommentCreated(CommentCreated { comment })])
  }
}

#[test]
fn test_comment() {
  database::db().shows.insert(
    "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    bits_data::Show {
      id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      creator_id: UserId::new(),
      name: Text::new("name"),
      started_at: None,
    },
  );

  let payload = CommentCommand::new(CommentInput {
    user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    text: Text::new("text"),
  })
  .handle()
  .unwrap();

  assert_json_snapshot!(payload, {"[0].comment.id" => "[uuid]"}, @r###"
  [
    {
      "event": "comment_created",
      "comment": {
        "id": "[uuid]",
        "user_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
        "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
        "text": "text"
      }
    }
  ]
  "###);
}
