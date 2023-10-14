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

pub trait Command {
  type Error;
  type Input;
  type Payload;

  fn new(input: Self::Input) -> Result<Self, Self::Error>
  where
    Self: Sized;

  fn events(&mut self) -> Result<Vec<Event>, Self::Error>;

  fn payload(&self) -> Result<Self::Payload, Self::Error>;

  fn run(&mut self) -> Result<Self::Payload, Self::Error> {
    let events = self.events()?;

    dispatch(events).ok();

    self.payload()
  }
}

pub(crate) fn dispatch(events: Vec<Event>) -> error::Result<()> {
  events.into_iter().try_for_each(|event| match event {
    Event::AuctionMarkedReady(evt) => {
      auction_marked_ready::auction_marked_ready(evt)
    }
    Event::AuctionProductCreated(evt) => {
      auction_product_created::auction_product_created(evt)
    }
    Event::AuctionRevived(evt) => auction_revived::auction_revived(evt),
    Event::AuctionStarted(evt) => auction_started::auction_started(evt),
    Event::BidCreated(evt) => bid_created::bid_created(evt),
    Event::CommentCreated { payload } => {
      comment_created::comment_created(payload)
    }
    Event::ProductCreated(evt) => product_created::product_created(evt),
    Event::ShowCreated(evt) => show_created::show_created(evt),
    Event::ShowStarted(evt) => show_started::show_started(evt),
  })
}
