use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::ShowStarted;

pub fn show_started(event: ShowStarted) -> Result<()> {
  database::db()
    .shows
    .insert(event.show.id, event.show.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.show.id))
}
