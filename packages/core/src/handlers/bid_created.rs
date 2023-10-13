use crate::database;
use crate::error::Result;
use bits_data::BidCreated;

pub fn bid_created(event: BidCreated) -> Result<()> {
  let bid = event.bid;

  database::db().bids.insert(bid.id, bid);

  Ok(())
}
