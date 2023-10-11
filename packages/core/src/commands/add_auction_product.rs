use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionMarkedReady;
use bits_data::AuctionProduct;
use bits_data::AuctionProductAdded;
use bits_data::AuctionProductId;
use bits_data::Event;
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
  pub product: AuctionProduct,
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("auction not found: {0}")]
  AuctionNotFound(AuctionId),
  #[error("product not found: {0}")]
  ProductNotFound(ProductId),
  #[error("auction not created")]
  NotCreated,
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

  let mut auction_aggregate = AuctionAggregate::new(auction);
  let mut product_aggregate = AuctionProductAggregate::new();

  let events = [
    product_aggregate.handle(Command::AddAuctionProduct(input))?,
    auction_aggregate.handle(Command::AddAuctionProduct(input))?,
  ]
  .concat();

  events.iter().cloned().for_each(|event| {
    auction_aggregate.apply(event.clone());
    product_aggregate.apply(event);
  });

  let auction = auction_aggregate.auction;
  let product = product_aggregate.auction_product.ok_or(Error::NotCreated)?;

  dispatch::dispatch(events).ok();

  Ok(AddAuctionProductPayload { auction, product })
}

enum Command {
  AddAuctionProduct(AddAuctionProductInput),
}

struct AuctionAggregate {
  auction: Auction,
}

impl AuctionAggregate {
  fn new(auction: Auction) -> Self {
    Self { auction }
  }

  fn handle(&self, command: Command) -> Result<Vec<Event>, Error> {
    match command {
      Command::AddAuctionProduct(input) => {
        if self.auction.ready_at.is_none() {
          return Ok(vec![]);
        }

        Ok(vec![Event::AuctionMarkedReady(AuctionMarkedReady {
          id: input.auction_id,
          ready_at: Utc::now(),
        })])
      }
    }
  }

  fn apply(&mut self, event: Event) {
    match event {
      Event::AuctionMarkedReady(event) => {
        self.auction.ready_at = Some(event.ready_at);
      }
      _ => unreachable!(),
    }
  }
}

struct AuctionProductAggregate {
  auction_product: Option<AuctionProduct>,
}

impl AuctionProductAggregate {
  fn new() -> Self {
    Self {
      auction_product: None,
    }
  }

  fn handle(&self, command: Command) -> Result<Vec<Event>, Error> {
    match command {
      Command::AddAuctionProduct(input) => {
        Ok(vec![Event::AuctionProductAdded(AuctionProductAdded {
          id: AuctionProductId::new(),
          auction_id: input.auction_id,
          product_id: input.product_id,
        })])
      }
    }
  }

  fn apply(&mut self, event: Event) {
    match event {
      Event::AuctionProductAdded(event) => {
        self.auction_product = Some(AuctionProduct {
          id: event.id,
          auction_id: event.auction_id,
          product_id: event.product_id,
        });
      }
      _ => unreachable!(),
    }
  }
}
