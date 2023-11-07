use super::Command;
use crate::dispatcher;
use crate::dispatcher::DispatchError;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::sea_orm::DbErr;
use bits_data::sea_orm::EntityTrait;
use bits_data::show;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::ShowStarted;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartInput {
  pub show_id: ShowId,
}

impl StartInput {
  pub fn type_name() -> &'static str {
    "StartInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("showId", TypeRef::named_nn(TypeRef::ID)))
  }
}

#[derive(Clone, Serialize)]
pub struct StartResult {
  pub show: Show,
}

impl StartResult {
  pub fn type_name() -> &'static str {
    "StartResult"
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
  Db(#[from] DbErr),
  #[error("internal: dispatch error")]
  Dispatch(#[from] DispatchError),
  #[error("show not created")]
  NotCreated,
  #[error("show not found")]
  NotFound,
}

pub struct StartCommand {
  show: Show,
}

impl Command for StartCommand {
  type Error = Error;
  type Event = Event;
  type Input = StartInput;
  type Result = StartResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::ShowStarted {
      data: ShowStarted {
        id: input.show_id,
        show: self.show.clone(),
      },
    }])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ShowStarted { data, .. } => Some(StartResult {
        show: Show {
          started: true,
          ..data.show.clone()
        },
      }),
      _ => None,
    })
  }
}

pub async fn start(
  client: &Client,
  input: StartInput,
) -> Result<StartResult, Error> {
  let show = show::Entity::find_by_id(input.show_id)
    .one(&client.connection)
    .await?
    .ok_or(Error::NotFound)?;

  let events = StartCommand { show }.handle(input)?;

  dispatcher::dispatch(client, events)
    .await
    .map(StartCommand::apply)
    .map_err(Error::Dispatch)?
    .ok_or(Error::NotCreated)
}
