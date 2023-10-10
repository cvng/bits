use crate::error::Result;
use crate::handlers::bid_placed;
use crate::handlers::comment_added;
use crate::handlers::product_created;
use crate::handlers::show_created;
use crate::handlers::show_marked_ready;
use crate::handlers::show_product_added;
use crate::handlers::show_started;
use bits_data::Event;

pub fn dispatch(events: Vec<Event>) -> Result<()> {
  events.into_iter().try_for_each(|event| match event {
    Event::BidPlaced(evt) => bid_placed::bid_placed(evt),
    Event::CommentAdded(evt) => comment_added::comment_added(evt),
    Event::ProductCreated(evt) => product_created::product_created(evt),
    Event::ShowCreated(evt) => show_created::show_created(evt),
    Event::ShowMarkedReady(evt) => show_marked_ready::show_marked_ready(evt),
    Event::ShowProductAdded(evt) => show_product_added::show_product_added(evt),
    Event::ShowStarted(evt) => show_started::show_started(evt),
  })
}
