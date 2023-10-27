use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::ShowCreated;

pub async fn show_created(_client: &Client, event: ShowCreated) -> Result<()> {
  database::db()
    .shows
    .insert(event.show.id, event.show.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.show.id))
}
