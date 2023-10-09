use crate::error::Result;
use crate::handlers;
use bits_data::Event;

pub fn dispatch(events: Vec<Event>) -> Result<()> {
  events.into_iter().try_for_each(|event| match event {
    Event::ShowCreated(event) => handlers::show_created::show_created(event),
    Event::ShowStarted(event) => handlers::show_started::show_started(event),
  })
}
