use crate::database;
use crate::error::Result;
use crate::Error;
use bits_data::AuctionMarkedReady;

pub fn auction_marked_ready(event: AuctionMarkedReady) -> Result<()> {
  let mut auction = database::db()
    .auctions
    .get(&event.id)
    .cloned()
    .ok_or_else(|| Error::NotFound(event.id.into()))?;

  auction.ready_at = Some(event.ready_at);

  database::db()
    .auctions
    .insert(auction.id, auction)
    .ok_or(Error::NotFound(auction.id.into()))?;

  Ok(())
}
