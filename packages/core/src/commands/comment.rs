use crate::command::Command;
use crate::dispatcher::DispatchError;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::comment;
use bits_data::sea_orm::EntityTrait;
use bits_data::Comment;
use bits_data::CommentCreated;
use bits_data::CommentId;
use bits_data::Event;
use bits_data::PersonId;
use bits_data::ShowId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
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
      .field(InputValue::new("authorId", TypeRef::named_nn(TypeRef::ID)))
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
  #[error("internal: db error")]
  Dx(#[from] DispatchError),
  #[error("comment {0:?} not found")]
  NotFound(CommentId),
}

pub struct CommentCommand<'a> {
  client: &'a Client,
}

impl<'a> Command for CommentCommand<'a> {
  type Error = Error;
  type Input = CommentInput;
  type Result = CommentResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    Ok(vec![Event::CommentCreated {
      data: CommentCreated {
        id: CommentId::new_v4(),
        author_id: input.author_id,
        show_id: input.show_id,
        text: input.text,
      },
    }])
  }

  async fn apply(
    &self,
    _input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let comment_id = events
      .iter()
      .find_map(|event| match event {
        Event::CommentCreated { data, .. } => Some(data.id),
        _ => None,
      })
      .unwrap();

    let comment = comment::Entity::find_by_id(comment_id)
      .one(&self.client.connection)
      .await
      .map_err(DispatchError::Database)?
      .ok_or(Error::NotFound(comment_id))?;

    Ok(Self::Result { comment })
  }
}

pub async fn comment(
  client: &Client,
  input: CommentInput,
) -> Result<CommentResult, Error> {
  CommentCommand { client }.run(input).await
}
