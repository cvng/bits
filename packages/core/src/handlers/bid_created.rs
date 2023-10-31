use crate::database;
use crate::decoder::insecure_get_token_sub;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::BidCreated;
use uuid::Uuid;

pub async fn bid_created(client: &Client, event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
    .ok();

  if let Some(token) = &client.token {
    dbg!(insecure_get_token_sub::<Uuid>(token.0.as_str())?);
  }

  Ok(())
}
