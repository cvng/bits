use crate::database;
use crate::Result;
use bits_data::ShowCreated;

pub fn show_created(event: ShowCreated) -> Result<()> {
  let ShowCreated { show } = event;

  database::db().shows.insert(show.id.clone(), show);

  Ok(())
}
