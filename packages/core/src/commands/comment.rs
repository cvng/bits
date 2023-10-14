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

#[derive(Copy, Clone, Serialize, InputObject)]
pub struct CommentInput {
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}

#[derive(Serialize, SimpleObject)]
pub struct CommentPayload {
  pub comment: Comment,
}

#[derive(Debug, Error)]
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

    Ok(vec![Event::CommentCreated {
      payload: CommentCreated { comment },
    }])
  }
}

#[test]
fn test_comment() {
  #[derive(Serialize)]
  struct Info {
    show: bits_data::Show,
    input: CommentInput,
  }

  let info = Info {
    show: bits_data::Show {
      id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      creator_id: UserId::new(),
      name: Text::new("name"),
      started_at: None,
    },
    input: CommentInput {
      user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
      show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      text: Text::new("text"),
    },
  };

  database::db().shows.insert(info.show.id, info.show);

  let payload = CommentCommand::new(info.input).handle().unwrap();

  with_settings!(
    { info => &info },
    { assert_json_snapshot!(payload, {"[0].payload.comment.id" => "[uuid]"}, @r###"
      [
        {
          "type": "comment_created",
          "payload": {
            "comment": {
              "id": "[uuid]",
              "user_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
              "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
              "text": "text"
            }
          }
        }
      ]
      "###) }
  );
}
