use crate::database;
use crate::error::Error;
use crate::error::Result;
use crate::Client;
use bits_data::AuctionCreated;

pub async fn auction_created(
  _client: &Client,
  event: AuctionCreated,
) -> Result<()> {
  database::db()
    .auctions
    .insert(event.auction.id, event.auction.clone())
    .map(|_| ())
    .ok_or(Error::NotFound(event.auction.id))
}
