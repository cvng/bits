use crate::database;
use crate::error::Result;
use bits_data::Bid;
use bits_data::BidPlaced;

pub fn bid_placed(event: BidPlaced) -> Result<()> {
  let bid = Bid {
    id: event.id,
    user_id: event.user_id,
    product_id: event.product_id,
    amount: event.amount,
  };

  database::db().bids.insert(bid.id, bid);

  Ok(())
}
