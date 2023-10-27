use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Context;
use bits_data::BidCreated;

pub async fn bid_created(_ctx: &Context, event: BidCreated) -> Result<()> {
  database::db()
    .bids
    .insert(event.bid.id, event.bid.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.bid.id))
}
