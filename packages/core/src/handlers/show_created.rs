use crate::database;
use crate::Result;
use bits_data::ShowCreated;

pub fn show_created(event: ShowCreated) -> Result<()> {
  let show = event.show;

  database::db()
    .shows
    .insert(show.id, show)
    .map(|_| Ok(()))
    .unwrap_or(Ok(()))
}
