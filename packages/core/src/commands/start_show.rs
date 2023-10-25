use crate::command::Command;
use crate::database;
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
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::DateTime;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Utc;
use thiserror::Error;

pub struct StartShowInput {
  pub id: ShowId,
}

impl StartShowInput {
  pub fn to_input_object() -> InputObject {
    InputObject::new("StartShowInput")
      .field(InputValue::new("id", TypeRef::named_nn(TypeRef::ID)))
  }
}

pub struct StartShowResult {
  pub show: Show,
}

impl StartShowResult {
  pub fn to_object() -> Object {
    Object::new("StartShowResult").field(Field::new(
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

impl From<StartShowResult> for Value {
  fn from(value: StartShowResult) -> Self {
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
  #[error("already started: {0}")]
  AlreadyStarted(ShowId),
  #[error("auction not found: {0}")]
  AuctionNotFound(ShowId),
  #[error("auction not ready: {0}")]
  AuctionNotReady(AuctionId),
}

pub struct StartShowCommand {
  pub now: DateTime,
  pub show: Option<Show>,
  pub auction: Option<Auction>,
}

impl Command for StartShowCommand {
  type Error = Error;
  type Event = Event;
  type Input = StartShowInput;
  type Result = StartShowResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let mut show = self.show.ok_or(Error::NotFound(input.id))?;

    let mut auction = self.auction.ok_or(Error::AuctionNotFound(show.id))?;

    // Check that the show hasn't already started.
    if show.started.is_some() {
      return Err(Error::AlreadyStarted(show.id));
    }

    let now = self.now;

    show.started = Some(now);
    auction.started = Some(now);

    Ok(vec![
      Event::show_started(show),
      Event::auction_started(auction),
    ])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ShowStarted { payload } => {
        Some(StartShowResult { show: payload.show })
      }
      _ => None,
    })
  }
}

pub async fn start_show(
  input: StartShowInput,
) -> Result<StartShowResult, Error> {
  let now = Utc::now();

  let show = database::db().shows.get(&input.id).cloned();

  let auction = show.and_then(|show| {
    database::db()
      .auctions
      .values()
      .find(|auction| auction.show_id == show.id)
      .cloned()
  });

  StartShowCommand { now, show, auction }
    .handle(input)
    .map(dispatcher::dispatch)?
    .map(StartShowCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_start_show() {
  let now = "2023-10-17T03:16:49.225067Z".parse().unwrap();

  let show = Some(Show {
    id: "441fdcfb-1613-4ed8-8d31-9fe8708680b0".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: "ba7220d5-af00-4815-89d3-5f852b733591".parse().unwrap(),
    name: "name".parse().unwrap(),
    started: None,
  });

  let product = Some(bits_data::Product {
    id: "e0c0e324-7b46-4020-ab33-bbbb91d26cfc".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: bits_data::UserId::new(),
    name: "name".parse().unwrap(),
  });

  let auction = Some(Auction {
    id: "a4d74d78-a628-4a0e-8e42-db3b4dca5f5c".parse().unwrap(),
    created: None,
    updated: None,
    show_id: show.as_ref().unwrap().id,
    product_id: product.as_ref().unwrap().id,
    started: Some("2023-10-17T02:55:11.788274Z".parse().unwrap()),
    expired: None,
  });

  let input = StartShowInput {
    id: show.as_ref().unwrap().id,
  };

  let events = StartShowCommand { now, show, auction }
    .handle(input)
    .unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "show_started",
      "payload": {
        "show": {
          "id": "441fdcfb-1613-4ed8-8d31-9fe8708680b0",
          "created": null,
          "updated": null,
          "creator_id": "ba7220d5-af00-4815-89d3-5f852b733591",
          "name": "name",
          "started": "2023-10-17T03:16:49.225067Z"
        }
      }
    },
    {
      "type": "auction_started",
      "payload": {
        "auction": {
          "id": "a4d74d78-a628-4a0e-8e42-db3b4dca5f5c",
          "created": null,
          "updated": null,
          "show_id": "441fdcfb-1613-4ed8-8d31-9fe8708680b0",
          "product_id": "e0c0e324-7b46-4020-ab33-bbbb91d26cfc",
          "started": "2023-10-17T03:16:49.225067Z",
          "expired": null
        }
      }
    }
  ]
  "###);
}
