use crate::error::Result;
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

pub fn dispatch(events: Vec<Event>) -> Result<Vec<Event>> {
  events
    .iter()
    .cloned()
    .try_for_each(|event| match event {
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
    .map(|_| events)
}
