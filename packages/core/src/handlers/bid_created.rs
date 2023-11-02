use crate::error::Result;
use crate::Client;
use bits_data::BidCreated;

pub async fn bid_created(_client: &Client, _event: BidCreated) -> Result<()> {
  Ok(())
}
