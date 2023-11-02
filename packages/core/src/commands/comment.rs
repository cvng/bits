use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::entities;
use bits_data::Comment;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::UserId;
use sea_orm::EntityTrait;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInput {
  pub user_id: UserId,
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
  #[error("show not found: {0}")]
  ShowNotFound(ShowId),
}

#[derive(Default)]
pub struct CommentCommand {
  pub show: Option<Show>,
  pub comment: Option<Comment>,
}

impl Command for CommentCommand {
  type Error = Error;
  type Event = Event;
  type Input = CommentInput;
  type Result = CommentResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    self
      .show
      .clone()
      .ok_or(Error::ShowNotFound(input.show_id))?;

    let comment = self.comment.clone().ok_or(Error::NotCreated)?;

    Ok(vec![Event::comment_created(comment)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::CommentCreated { data } => Some(CommentResult {
        comment: data.comment.clone(),
      }),
      _ => None,
    })
  }
}

pub async fn comment(
  client: &Client,
  input: CommentInput,
) -> Result<CommentResult, Error> {
  let show = entities::prelude::Show::find_by_id(input.show_id)
    .one(&client.connection)
    .await
    .map_err(|_| Error::ShowNotFound(input.show_id))?;

  let comment = Some(Comment {
    id: CommentId::new_v4(),
    created: None,
    updated: None,
    author_id: input.user_id,
    show_id: input.show_id,
    text: input.text.clone(),
  });

  dispatcher::dispatch(client, CommentCommand { show, comment }.handle(input)?)
    .await
    .map(CommentCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_comment() {
  let show = Some(bits_data::Show {
    id: "f5e84179-7f8d-461b-a1d9-497974de10a6".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: UserId::new_v4(),
    name: "name".parse().unwrap(),
    started: None,
  });

  let input = CommentInput {
    user_id: "9ad4e977-8156-450e-ad00-944f9fc730ab".parse().unwrap(),
    show_id: show.as_ref().unwrap().id,
    text: "text".parse().unwrap(),
  };

  let comment = Some(Comment {
    id: "7cc32b32-c5c6-4034-89f9-8363d856ebb4".parse().unwrap(),
    created: None,
    updated: None,
    author_id: input.user_id,
    show_id: input.show_id,
    text: input.text.clone(),
  });

  let events = CommentCommand { show, comment }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "comment_created",
      "data": {
        "comment": {
          "id": "7cc32b32-c5c6-4034-89f9-8363d856ebb4",
          "created": null,
          "updated": null,
          "author_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
          "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
          "text": "text"
        },
        "id": "7cc32b32-c5c6-4034-89f9-8363d856ebb4",
        "author_id": "9ad4e977-8156-450e-ad00-944f9fc730ab",
        "show_id": "f5e84179-7f8d-461b-a1d9-497974de10a6",
        "text": "text"
      }
    }
  ]
  "###);
}
