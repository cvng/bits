use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::AuctionRevived;

pub fn auction_revived(event: AuctionRevived) -> Result<()> {
  database::db()
    .auctions
    .insert(event.auction.id, event.auction)
    .map(|_| ())
    .ok_or(Error::NotFound(event.auction.id.into()))
}
