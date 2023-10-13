use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::AuctionStarted;

pub fn auction_started(event: AuctionStarted) -> Result<()> {
  let mut auction = database::db()
    .auctions
    .get(&event.id)
    .cloned()
    .ok_or_else(|| Error::NotFound(event.id.into()))?;

  auction.started_at = Some(event.started_at);

  database::db().auctions.insert(auction.id, auction);

  Ok(())
}
