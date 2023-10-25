use crate::command::Command;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Text;
use bits_data::UserId;
use thiserror::Error;

#[derive(InputObject)]
pub struct CreateShowInput {
  pub creator_id: UserId,
  pub name: Text,
}

#[derive(SimpleObject)]
pub struct CreateShowResult {
  pub show: Show,
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
    creator_id: input.creator_id,
    name: input.name,
    started_at: None,
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
    name: "name".parse().unwrap()
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
