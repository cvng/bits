use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::ShowStarted;

pub fn show_started(event: ShowStarted) -> Result<()> {
  let mut show = database::db()
    .shows
    .get(&event.id)
    .cloned()
    .ok_or_else(|| Error::NotFound(event.id.into()))?;

  show.started_at = Some(event.started_at);

  database::db()
    .shows
    .insert(show.id, show)
    .ok_or(Error::NotFound(show.id.into()))?;

  Ok(())
}
