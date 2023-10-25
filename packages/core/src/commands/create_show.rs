use crate::command::Command;
use crate::dispatcher;
use async_graphql::dynamic::indexmap::IndexMap;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_graphql::Name;
use async_graphql::Value;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use thiserror::Error;

pub struct CreateShowInput {
  pub creator_id: UserId,
  pub name: Text,
}

impl CreateShowInput {
  pub fn to_input_object() -> InputObject {
    InputObject::new("CreateShowInput")
      .field(InputValue::new("creatorId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

pub struct CreateShowResult {
  pub show: Show,
}

impl CreateShowResult {
  pub fn to_object() -> Object {
    Object::new("CreateShowResult").field(Field::new(
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

impl From<CreateShowResult> for Value {
  fn from(value: CreateShowResult) -> Self {
    let mut map = IndexMap::new();
    map.insert(Name::new("id"), value.show.id.to_string().into());
    Value::Object(map)
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
    let show = self.show.ok_or(Error::NotCreated)?;

    Ok(vec![Event::show_created(show)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ShowCreated { payload } => {
        Some(CreateShowResult { show: payload.show })
      }
      _ => None,
    })
  }
}

pub async fn create_show(
  input: CreateShowInput,
) -> Result<CreateShowResult, Error> {
  let show = Some(Show {
    id: ShowId::new(),
    created: None,
    updated: None,
    creator_id: input.creator_id,
    name: input.name,
    started: None,
  });

  CreateShowCommand { show }
    .handle(input)
    .map(dispatcher::dispatch)?
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
    creator_id: input.creator_id,
    name: input.name,
    started_at: None,
  });

  let events = CreateShowCommand { show }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "show_created",
      "payload": {
        "show": {
          "id": "15f4491c-c0ab-437e-bdfd-60a62ad8c857",
          "creator_id": "d9bd7c14-d793-47f3-a644-f97921c862ed",
          "name": "name",
          "started_at": null
        }
      }
    }
  ]
  "###);
}
