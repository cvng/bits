use super::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Comment;
use bits_data::CommentCreated;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::PersonId;
use bits_data::ShowId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInput {
  pub author_id: PersonId,
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

#[derive(Clone, Serialize)]
pub struct CommentResult {
  pub comment: Comment,
}

impl CommentResult {
  pub fn type_name() -> &'static str {
    "CommentResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "comment",
      TypeRef::named_nn("Comment"),
      |ctx| {
        FieldFuture::new(async move {
          Ok(Some(FieldValue::owned_any(
            ctx
              .parent_value
              .try_downcast_ref::<Self>()
              .cloned()
              .unwrap()
              .comment,
          )))
        })
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
    Ok(vec![Event::CommentCreated {
      data: CommentCreated {
        id: CommentId::new_v4(),
        author_id: input.author_id,
        show_id: input.show_id,
        text: input.text,
      },
    }])
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
  let events = CommentCommand {}.handle(input)?;

  dispatcher::dispatch(client, events)
    .await
    .map(CommentCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_comment() {
  let input = CommentInput {
    author_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    text: "text".to_string(),
  };

  let events = CommentCommand {}.handle(input).unwrap();

  insta::assert_json_snapshot!(events, { "[0].data.id" => "[uuid]" }, @r###"
  [
    {
      "type": "comment_created",
      "data": {
        "id": "[uuid]",
        "author_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
        "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
        "text": "text"
      }
    }
  ]
  "###);
}
