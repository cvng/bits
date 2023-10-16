use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Comment;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::Show;
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
  #[error("comment not created")]
  NotCreated,
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

#[derive(Default)]
struct CommentCommand {
  show: Option<Show>,
  comment: Option<Comment>,
}

impl CommentCommand {
  fn new(input: &CommentInput) -> Self {
    let show = database::db().shows.get(&input.show_id).cloned();

    let comment = Some(Comment {
      id: CommentId::new(),
      user_id: input.user_id,
      show_id: input.show_id,
      text: input.text,
    });

    Self { show, comment }
  }

  fn run(&self, input: CommentInput) -> Result<CommentPayload, Error> {
    self
      .handle(input)
      .map(|events| dispatcher::dispatch(events).unwrap())
      .map(|events| CommentCommand::apply(events).unwrap())
  }
}

impl Command for CommentCommand {
  type Error = Error;
  type Event = Event;
  type Input = CommentInput;
  type Payload = CommentPayload;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    self.show.ok_or(Error::ShowNotFound(input.show_id))?;

    let comment = self.comment.ok_or(Error::NotCreated)?;

    Ok(vec![Event::comment_created(comment)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Payload> {
    events.iter().fold(None, |_, event| match event {
      Event::CommentCreated { payload } => Some(CommentPayload {
        comment: payload.comment,
      }),
      _ => None,
    })
  }
}

pub fn comment(input: CommentInput) -> Result<CommentPayload, Error> {
  CommentCommand::new(&input).run(input)
}

#[test]
fn test_comment() {
  let show = bits_data::Show {
    id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    creator_id: UserId::new(),
    name: Text::new("name"),
    started_at: None,
  };

  let input = CommentInput {
    user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: show.id,
    text: Text::new("text"),
  };

  let comment = Comment {
    id: "7cc32b32-c5c6-4034-89f9-8363d856ebb4".parse().unwrap(),
    user_id: input.user_id,
    show_id: input.show_id,
    text: input.text,
  };

  let events = CommentCommand {
    show: Some(show),
    comment: Some(comment),
  }
  .handle(input)
  .unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "comment_created",
      "payload": {
        "comment": {
          "id": "7cc32b32-c5c6-4034-89f9-8363d856ebb4",
          "user_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
          "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
          "text": "text"
        }
      }
    }
  ]
  "###);
}
