use crate::database;
use crate::error::Result;
use crate::Error;
use bits_data::ShowMarkedReady;

pub fn show_marked_ready(event: ShowMarkedReady) -> Result<()> {
  let mut show = database::db()
    .shows
    .get(&event.id)
    .cloned()
    .ok_or_else(|| Error::NotFound(event.id.into()))?;

  show.ready_at = Some(event.ready_at);

  database::db()
    .shows
    .insert(show.id, show)
    .ok_or(Error::NotFound(show.id.into()))?;

  Ok(())
}
