use crate::error;
use crate::handlers::auction_marked_ready;
use crate::handlers::auction_product_created;
use crate::handlers::auction_revived;
use crate::handlers::auction_started;
use crate::handlers::bid_created;
use crate::handlers::comment_created;
use crate::handlers::product_created;
use crate::handlers::show_created;
use crate::handlers::show_started;
use bits_data::Event;

pub(crate) trait Command {
  type Error;
  type State;
  type Input;
  type Payload;

  fn run(&self, input: Self::Input) -> Result<Self::Payload, Self::Error> {
    let state = self.state(&input)?;

    let events = self.events(&state, &input)?;

    crate::dispatch::dispatch(events.clone()).ok();

    let payload = self.payload(events).unwrap(); // TODO

    Ok(payload)
  }

  fn state(&self, input: &Self::Input) -> Result<Self::State, Self::Error>;

  fn events(
    &self,
    state: &Self::State,
    input: &Self::Input,
  ) -> Result<Vec<Event>, Self::Error>;

  fn payload(&self, events: Vec<Event>) -> Option<Self::Payload>;
}

pub(crate) fn dispatch(events: Vec<Event>) -> error::Result<()> {
  events.into_iter().try_for_each(|event| match event {
    Event::AuctionMarkedReady(evt) => {
      auction_marked_ready::auction_marked_ready(evt)
    }
    Event::AuctionProductCreated(evt) => {
      auction_product_created::auction_product_created(evt)
    }
    Event::AuctionRevived { payload } => {
      auction_revived::auction_revived(payload)
    }
    Event::AuctionStarted(evt) => auction_started::auction_started(evt),
    Event::BidCreated { payload } => bid_created::bid_created(payload),
    Event::CommentCreated { payload } => {
      comment_created::comment_created(payload)
    }
    Event::ProductCreated(evt) => product_created::product_created(evt),
    Event::ShowCreated(evt) => show_created::show_created(evt),
    Event::ShowStarted(evt) => show_started::show_started(evt),
  })
}
