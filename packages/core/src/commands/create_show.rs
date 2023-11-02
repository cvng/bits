use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::UserId;
use thiserror::Error;

#[derive(Deserialize)]
pub struct CreateShowInput {
  #[serde(rename = "creatorId")]
  pub creator_id: UserId,
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

#[derive(Serialize)]
pub struct CreateShowResult {
  pub show: Show,
}

impl CreateShowResult {
  pub fn type_name() -> &'static str {
    "CreateShowResult"
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
  #[error("show not created")]
  NotCreated,
  #[error("not found: {0}")]
  NotFound(ShowId),
}

pub struct CreateShowCommand {
  pub show: Option<Show>,
}

impl Command for CreateShowCommand {
  type Error = Error;
  type Event = Event;
  type Input = CreateShowInput;
  type Result = CreateShowResult;

  fn handle(
    &self,
    _input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let show = self.show.clone().ok_or(Error::NotCreated)?;

    Ok(vec![Event::show_created(show)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ShowCreated { payload } => Some(CreateShowResult {
        show: payload.show.clone(),
      }),
      _ => None,
    })
  }
}

pub async fn create_show(
  client: &Client,
  input: CreateShowInput,
) -> Result<CreateShowResult, Error> {
  let show = Some(Show {
    id: ShowId::new_v4(),
    created: None,
    updated: None,
    creator_id: input.creator_id,
    name: input.name.clone(),
    started: None,
  });

  dispatcher::dispatch(client, CreateShowCommand { show }.handle(input)?)
    .await
    .map(CreateShowCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_show() {
  let input = CreateShowInput {
    creator_id: "d9bd7c14-d793-47f3-a644-f97921c862ed".parse().unwrap(),
    name: "name".parse().unwrap(),
  };

  let show = Some(Show {
    id: "15f4491c-c0ab-437e-bdfd-60a62ad8c857".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: input.creator_id,
    name: input.name.clone(),
    started: None,
  });

  let events = CreateShowCommand { show }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "show_created",
      "payload": {
        "show": {
          "id": "15f4491c-c0ab-437e-bdfd-60a62ad8c857",
          "created": null,
          "updated": null,
          "creator_id": "d9bd7c14-d793-47f3-a644-f97921c862ed",
          "name": "name",
          "started": null
        }
      }
    }
  ]
  "###);
}
