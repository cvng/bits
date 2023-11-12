use crate::command::Command;
use crate::dispatcher::InternalError;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::sea_orm::EntityTrait;
use bits_data::show;
use bits_data::Event;
use bits_data::PersonId;
use bits_data::Show;
use bits_data::ShowCreated;
use bits_data::ShowId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateShowInput {
  pub creator_id: PersonId,
  pub name: String,
}

impl CreateShowInput {
  pub fn type_name() -> &'static str {
    "CreateShowInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("creatorId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

#[derive(Clone, Serialize)]
pub struct CreateShowResult {
  pub show: Show,
}

impl CreateShowResult {
  pub fn type_name() -> &'static str {
    "CreateShowResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "show",
      TypeRef::named_nn("Show"),
      |ctx| {
        FieldFuture::new(async move {
          Ok(Some(FieldValue::owned_any(
            ctx
              .parent_value
              .try_downcast_ref::<Self>()
              .cloned()
              .unwrap()
              .show,
          )))
        })
      },
    ))
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("internal: db error")]
  Internal(#[from] InternalError),
  #[error("show {0:?} not found")]
  NotFound(ShowId),
}

pub struct CreateShowCommand<'a> {
  pub client: &'a Client,
}

impl<'a> Command for CreateShowCommand<'a> {
  type Error = Error;
  type Input = CreateShowInput;
  type Result = CreateShowResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    Ok(vec![Event::ShowCreated {
      data: ShowCreated {
        id: ShowId::new_v4(),
        creator_id: input.creator_id,
        name: input.name,
      },
    }])
  }

  async fn apply(
    &self,
    _input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let show_id = events
      .iter()
      .find_map(|event| match event {
        Event::ShowCreated { data, .. } => Some(data.id),
        _ => None,
      })
      .unwrap();

    let show = show::Entity::find_by_id(show_id)
      .one(&self.client.connection)
      .await
      .map_err(InternalError::Database)?
      .ok_or(Error::NotFound(show_id))?;

    Ok(Self::Result { show })
  }
}

pub async fn create_show(
  client: &Client,
  input: CreateShowInput,
) -> Result<CreateShowResult, Error> {
  CreateShowCommand { client }.run(input).await
}
