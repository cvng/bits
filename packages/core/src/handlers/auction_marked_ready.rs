use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::AuctionMarkedReady;

pub fn auction_marked_ready(event: AuctionMarkedReady) -> Result<()> {
  database::db()
    .auctions
    .insert(event.auction.id, event.auction)
    .map(|_| ())
    .ok_or(Error::NotFound(event.auction.id.into()))
}
