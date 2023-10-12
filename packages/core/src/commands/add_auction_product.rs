use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionMarkedReady;
use bits_data::AuctionProductAdded;
use bits_data::AuctionProductId;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Utc;
use thiserror::Error;

#[derive(Clone, Copy, InputObject)]
pub struct AddAuctionProductInput {
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

#[derive(SimpleObject)]
pub struct AddAuctionProductPayload {
  pub auction: Auction,
  pub product: Product,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("auction not found: {0}")]
  AuctionNotFound(AuctionId),
  #[error("product not found: {0}")]
  ProductNotFound(ProductId),
}

struct State {
  auction: Auction,
}

fn run_add_auction_product(
  state: &State,
  input: AddAuctionProductInput,
) -> Result<Vec<Event>, Error> {
  let mut events = vec![];

  events.push(
    AuctionProductAdded {
      id: AuctionProductId::new(),
      auction_id: input.auction_id,
      product_id: input.product_id,
    }
    .into(),
  );

  if state.auction.ready_at.is_none() {
    events.push(
      AuctionMarkedReady {
        id: state.auction.id,
        ready_at: Utc::now(),
      }
      .into(),
    )
  }

  Ok(events)
}

pub async fn add_auction_product(
  input: AddAuctionProductInput,
) -> Result<AddAuctionProductPayload, Error> {
  let auction = database::db()
    .auctions
    .get(&input.auction_id)
    .cloned()
    .ok_or(Error::AuctionNotFound(input.auction_id))?;

  database::db()
    .products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  let state = State { auction };

  dispatch::dispatch(run_add_auction_product(&state, input)?).ok();

  let auction = database::db()
    .auctions
    .get(&input.auction_id)
    .cloned()
    .ok_or(Error::AuctionNotFound(input.auction_id))?;

  let product = database::db()
    .products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  Ok(AddAuctionProductPayload { auction, product })
}
