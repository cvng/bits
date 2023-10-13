use crate::database;
use crate::error::Result;
use crate::Error;
use bits_data::AuctionRevived;

pub fn auction_revived(event: AuctionRevived) -> Result<()> {
  let mut auction = database::db()
    .auctions
    .get(&event.id)
    .cloned()
    .ok_or(Error::NotFound(event.id.into()))?;

  auction.expired_at = Some(event.expired_at);

  database::db().auctions.insert(auction.id, auction);

  Ok(())
}
