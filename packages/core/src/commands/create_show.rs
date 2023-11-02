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
#[serde(rename_all = "camelCase")]
pub struct CreateShowInput {
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
}

pub struct CreateShowCommand {}

impl Command for CreateShowCommand {
  type Error = Error;
  type Event = Event;
  type Input = CreateShowInput;
  type Result = CreateShowResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    Ok(vec![Event::show_created(
      ShowId::new_v4(),
      input.creator_id,
      input.name,
    )])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ShowCreated { data, .. } => Some(CreateShowResult {
        show: Show {
          id: data.id,
          created: None,
          updated: None,
          creator_id: data.creator_id,
          name: data.name.clone(),
          started: None,
        },
      }),
      _ => None,
    })
  }
}

pub async fn create_show(
  client: &Client,
  input: CreateShowInput,
) -> Result<CreateShowResult, Error> {
  let events = CreateShowCommand {}.handle(input)?;

  dispatcher::dispatch(client, events)
    .await
    .map(CreateShowCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_show() {
  let input = CreateShowInput {
    creator_id: "d9bd7c14-d793-47f3-a644-f97921c862ed".parse().unwrap(),
    name: "name".to_string(),
  };

  let events = CreateShowCommand {}.handle(input).unwrap();

  assert_json_snapshot!(events, { "[0].data.id" => "[uuid]" }, @r###"
  [
    {
      "type": "show_created",
      "data": {
        "id": "[uuid]",
        "creator_id": "d9bd7c14-d793-47f3-a644-f97921c862ed",
        "name": "name"
      }
    }
  ]
  "###);
}
