use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::Event;
use bits_data::Show;
use bits_data::ShowId;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct StartShowInput {
  pub id: ShowId,
}

#[derive(SimpleObject)]
pub struct StartShowResult {
  pub show: Show,
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
    if show.started_at.is_some() {
      return Err(Error::AlreadyStarted(show.id));
    }

    // Check that the auction is ready.
    if auction.ready_at.is_none() {
      return Err(Error::AuctionNotReady(auction.id));
    }

    let now = Utc::now();

    show.started_at = Some(now);
    auction.started_at = Some(now);

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
  let show = database::db().shows.get(&input.id).cloned();

  let auction = show.and_then(|show| {
    database::db()
      .auctions
      .values()
      .find(|auction| auction.show_id == show.id)
      .cloned()
  });

  StartShowCommand { show, auction }
    .handle(input)
    .map(dispatcher::dispatch)?
    .map(StartShowCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_start_show() {
  let show = Some(Show {
    id: "441fdcfb-1613-4ed8-8d31-9fe8708680b0".parse().unwrap(),
    creator_id: "ba7220d5-af00-4815-89d3-5f852b733591".parse().unwrap(),
    name: bits_data::Text::new("name"),
    started_at: Some("2023-10-17T02:55:11.788274Z".parse().unwrap()),
  });

  let auction = Some(Auction {
    id: "a4d74d78-a628-4a0e-8e42-db3b4dca5f5c".parse().unwrap(),
    show_id: show.as_ref().unwrap().id,
    ready_at: Some("2023-10-17T02:55:11.787768Z".parse().unwrap()),
    started_at: Some("2023-10-17T02:55:11.788274Z".parse().unwrap()),
    expired_at: None,
  });

  let input = StartShowInput {
    id: show.as_ref().unwrap().id,
  };

  let events = StartShowCommand { show, auction }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "show_started",
      "payload": {
        "show": {
          "id": "62e107a1-3192-4599-8084-d96c8adaf19c",
          "creator_id": "9a23b867-21d2-4bad-bce7-31bc6d405fa4",
          "name": "name",
          "started_at": "2023-10-17T03:00:15.068256Z"
        }
      }
    },
    {
      "type": "auction_started",
      "payload": {
        "auction": {
          "id": "83cb3f21-7a8c-4aa4-adf6-c708842dc4d2",
          "show_id": "62e107a1-3192-4599-8084-d96c8adaf19c",
          "ready_at": "2023-10-17T03:00:15.068093Z",
          "started_at": "2023-10-17T03:00:15.068256Z",
          "expired_at": null
        }
      }
    }
  ]
  "###);
}
