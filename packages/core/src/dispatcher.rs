use crate::error::Result;
use crate::handlers::auction_created;
use crate::handlers::auction_revived;
use crate::handlers::auction_started;
use crate::handlers::bid_created;
use crate::handlers::comment_created;
use crate::handlers::product_created;
use crate::handlers::show_created;
use crate::handlers::show_started;
use crate::Context;
use bits_data::Event;

pub async fn dispatch(ctx: &Context, events: Vec<Event>) -> Result<Vec<Event>> {
  for event in events.iter().cloned() {
    match event {
      Event::AuctionCreated { payload } => {
        auction_created::auction_created(ctx, payload).await?
      }
      Event::AuctionRevived { payload } => {
        auction_revived::auction_revived(ctx, payload).await?
      }
      Event::AuctionStarted { payload } => {
        auction_started::auction_started(ctx, payload).await?
      }
      Event::BidCreated { payload } => {
        bid_created::bid_created(ctx, payload).await?
      }
      Event::CommentCreated { payload } => {
        comment_created::comment_created(ctx, payload).await?
      }
      Event::ProductCreated { payload } => {
        product_created::product_created(ctx, payload).await?
      }
      Event::ShowCreated { payload } => {
        show_created::show_created(ctx, payload).await?
      }
      Event::ShowStarted { payload } => {
        show_started::show_started(ctx, payload).await?
      }
    }
  }

  Ok(events)
}
