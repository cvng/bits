use crate::client::insecure_get_token_user;
use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::BidCreated;

pub async fn bid_created(client: &Client, event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
    .ok();

  if let Some(token) = &client.token {
    dbg!(insecure_get_token_user(token)?);
  }

  Ok(())
}
