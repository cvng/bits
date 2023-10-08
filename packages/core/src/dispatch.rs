use crate::handlers;
use crate::Result;
use bits_data::Event;

pub fn dispatch(events: Vec<Event>) -> Result<()> {
    events.into_iter().try_for_each(|event| match event {
        Event::ShowCreated(event) => handlers::show_created(event),
    })
}
