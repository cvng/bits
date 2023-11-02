use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Comment;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::ShowId;
use bits_data::UserId;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInput {
  pub author_id: UserId,
  pub show_id: ShowId,
  pub text: String,
}

impl CommentInput {
  pub fn type_name() -> &'static str {
    "CommentInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("userId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("showId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("text", TypeRef::named_nn(TypeRef::STRING)))
  }
}

#[derive(Serialize)]
pub struct CommentResult {
  pub comment: Comment,
}

impl CommentResult {
  pub fn type_name() -> &'static str {
    "CommentResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "id".to_string(),
      TypeRef::named_nn(TypeRef::ID),
      |ctx| {
        FieldFuture::new(
          async move { Ok(ctx.parent_value.as_value().cloned()) },
        )
      },
    ))
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("comment not created")]
  NotCreated,
}

#[derive(Default)]
pub struct CommentCommand {}

impl Command for CommentCommand {
  type Error = Error;
  type Event = Event;
  type Input = CommentInput;
  type Result = CommentResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::comment_created(
      CommentId::new_v4(),
      input.author_id,
      input.show_id,
      input.text,
    )])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::CommentCreated { data, .. } => Some(CommentResult {
        comment: Comment {
          id: data.id,
          created: None,
          updated: None,
          author_id: data.author_id,
          show_id: data.show_id,
          text: data.text.clone(),
        },
      }),
      _ => None,
    })
  }
}

pub async fn comment(
  client: &Client,
  input: CommentInput,
) -> Result<CommentResult, Error> {
  dispatcher::dispatch(client, CommentCommand {}.handle(input)?)
    .await
    .map(CommentCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_comment() {
  let show = bits_data::Show {
    id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: UserId::new_v4(),
    name: "name".parse().unwrap(),
    started: None,
  };

  let input = CommentInput {
    author_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: show.id,
    text: "text".parse().unwrap(),
  };

  let events = CommentCommand {}.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "comment_created",
      "data": {
        "id": "a724e819-7a10-4343-9dfc-3b56b2c6d7be",
        "author_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
        "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
        "text": "text"
      }
    }
  ]
  "###);
}
