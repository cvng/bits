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

pub trait Command {
  type Error;
  type Input;
  type Payload;

  fn run(&mut self, input: Self::Input) -> Result<Self::Payload, Self::Error>;
  fn handle(&mut self, input: Self::Input) -> Result<Vec<Event>, Self::Error>;
}

#[derive(Default)]
pub struct CommentCommand {
  pub comment: Option<Comment>,
}

impl CommentCommand {
  pub fn new() -> Self {
    Self { comment: None }
  }
}

impl Command for CommentCommand {
  type Error = Error;
  type Input = CommentInput;
  type Payload = CommentPayload;

  fn run(&mut self, input: CommentInput) -> Result<CommentPayload, Error> {
    database::db()
      .shows
      .get(&input.show_id)
      .cloned()
      .ok_or(Error::ShowNotFound(input.show_id))?;

    dispatch::dispatch(self.handle(input)?).ok();

    Ok(CommentPayload {
      comment: self.comment.unwrap(),
    })
  }

  fn handle(&mut self, input: CommentInput) -> Result<Vec<Event>, Error> {
    let comment = Comment {
      id: CommentId::new(),
      user_id: input.user_id,
      show_id: input.show_id,
      text: input.text,
    };

    self.comment = Some(comment);

    Ok(vec![Event::CommentCreated(CommentCreated { comment })])
  }
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

  let payload = CommentCommand::new()
    .handle(CommentInput {
      user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
      show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      text: Text::new("text"),
    })
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
