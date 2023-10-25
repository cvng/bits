use crate::database;
use crate::error::Error;
use crate::error::Result;
use bits_data::BidCreated;

pub fn bid_created(event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
}
