use crate::database;
use crate::dispatch::Command;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Comment;
use bits_data::CommentCreated;
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
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

#[derive(Serialize)]
pub struct State {
  show: Show,
}

struct CommentCommand;

impl Command for CommentCommand {
  type Error = Error;
  type State = State;
  type Input = CommentInput;
  type Payload = CommentPayload;

  fn state(&self, input: &Self::Input) -> Result<Self::State, Self::Error> {
    Ok(State {
      show: database::db()
        .shows
        .get(&input.show_id)
        .cloned()
        .ok_or(Error::ShowNotFound(input.show_id))?,
    })
  }

  fn events(
    &self,
    _state: &Self::State,
    input: &Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    let comment = Comment {
      id: CommentId::new(),
      user_id: input.user_id,
      show_id: input.show_id,
      text: input.text,
    };

    Ok(vec![Event::CommentCreated {
      payload: CommentCreated { comment },
    }])
  }

  fn payload(&self, events: Vec<Event>) -> Option<Self::Payload> {
    events.iter().fold(None, |_, event| match event {
      Event::CommentCreated { payload } => Some(CommentPayload {
        comment: payload.comment,
      }),
      _ => None,
    })
  }
}

pub fn comment(input: CommentInput) -> Result<CommentPayload, Error> {
  CommentCommand.run(input)
}

#[test]
fn test_comment() {
  #[derive(Serialize)]
  struct Info {
    state: State,
    input: CommentInput,
  }

  let info = Info {
    state: State {
      show: bits_data::Show {
        id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
        creator_id: UserId::new(),
        name: Text::new("name"),
        started_at: None,
      },
    },
    input: CommentInput {
      user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
      show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
      text: Text::new("text"),
    },
  };

  database::db()
    .shows
    .insert(info.state.show.id, info.state.show);

  let events = CommentCommand.events(&info.state, &info.input).unwrap();

  with_settings!(
    { info => &info },
    { assert_json_snapshot!(events, {"[0].payload.comment.id" => "[uuid]"}, @r###"
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
