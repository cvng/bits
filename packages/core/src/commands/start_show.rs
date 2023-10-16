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
    let show = self.show.ok_or(Error::NotFound(input.id))?;

    let auction = self.auction.ok_or(Error::AuctionNotFound(show.id))?;

    // Check that the show hasn't already started.
    if show.started_at.is_some() {
      return Err(Error::AlreadyStarted(show.id));
    }

    // Check that the auction is ready.
    if auction.ready_at.is_none() {
      return Err(Error::AuctionNotReady(auction.id));
    }

    let now = Utc::now();

    Ok(vec![
      Event::show_started(show, now),
      Event::auction_started(auction.id, now),
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
