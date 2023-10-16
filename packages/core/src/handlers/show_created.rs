use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::ShowCreated;

pub fn show_created(event: ShowCreated) -> Result<()> {
  database::db()
    .shows
    .insert(event.show.id, event.show)
    .map(|_| ())
    .ok_or(Error::NotFound(event.show.id.into()))
}
