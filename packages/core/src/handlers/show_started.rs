use crate::database;
use crate::Result;
use bits_data::ShowStarted;

pub fn show_started(event: ShowStarted) -> Result<()> {
  let ShowStarted { show } = event;

  database::db().shows.insert(show.id, show);

  Ok(())
}
