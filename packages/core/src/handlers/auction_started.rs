use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Context;
use bits_data::AuctionStarted;

pub async fn auction_started(
  _ctx: &Context,
  event: AuctionStarted,
) -> Result<()> {
  database::db()
    .auctions
    .insert(event.auction.id, event.auction.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.auction.id))
}
