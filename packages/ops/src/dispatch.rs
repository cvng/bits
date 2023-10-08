use crate::show_created::show_created;
use crate::Event;
use crate::Result;

pub fn dispatch(events: Vec<Event>) -> Result<()> {
    events.into_iter().try_for_each(|event| match event {
        Event::ShowCreated(event) => show_created(event),
    })
}
