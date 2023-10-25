use crate::error::Result;
use crate::handlers::auction_created;
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
      Event::AuctionCreated { payload } => {
        auction_created::auction_created(payload)
      }
      Event::AuctionRevived { payload } => {
        auction_revived::auction_revived(payload)
      }
      Event::AuctionStarted { payload } => {
        auction_started::auction_started(payload)
      }
      Event::BidCreated { payload } => bid_created::bid_created(payload),
      Event::CommentCreated { payload } => {
        comment_created::comment_created(payload)
      }
      Event::ProductCreated { payload } => {
        product_created::product_created(payload)
      }
      Event::ShowCreated { payload } => show_created::show_created(payload),
      Event::ShowStarted { payload } => show_started::show_started(payload),
    })
    .map(|_| events)
}
