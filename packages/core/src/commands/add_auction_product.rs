use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionProduct;
use bits_data::AuctionProductId;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct AddAuctionProductInput {
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

#[derive(SimpleObject)]
pub struct AddAuctionProductResult {
  pub auction_product: AuctionProduct,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("auction not found: {0}")]
  AuctionNotFound(AuctionId),
  #[error("product not found: {0}")]
  ProductNotFound(ProductId),
  #[error("auction product not created")]
  NotCreated,
}

pub struct AddAuctionProductCommand {
  pub auction: Option<Auction>,
  pub product: Option<Product>,
  pub auction_product: Option<AuctionProduct>,
}

impl Command for AddAuctionProductCommand {
  type Error = Error;
  type Event = Event;
  type Input = AddAuctionProductInput;
  type Result = AddAuctionProductResult;

  fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let mut events = vec![];

    let mut auction = self
      .auction
      .ok_or(Error::AuctionNotFound(input.auction_id))?;

    self
      .product
      .ok_or(Error::ProductNotFound(input.product_id))?;

    let auction_product = self.auction_product.ok_or(Error::NotCreated)?;

    events.push(Event::auction_product_created(auction_product));

    if auction.ready_at.is_none() {
      auction.ready_at = Some(Utc::now());
      events.push(Event::auction_marked_ready(auction));
    }

    Ok(events)
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::AuctionProductCreated { payload } => {
        Some(AddAuctionProductResult {
          auction_product: payload.auction_product,
        })
      }
      _ => None,
    })
  }
}

pub async fn add_auction_product(
  input: AddAuctionProductInput,
) -> Result<AddAuctionProductResult, Error> {
  let auction = database::db().auctions.get(&input.auction_id).cloned();

  let product = database::db().products.get(&input.product_id).cloned();

  let auction_product = Some(AuctionProduct {
    id: AuctionProductId::new(),
    auction_id: input.auction_id,
    product_id: input.product_id,
    best_bid_id: None,
  });

  AddAuctionProductCommand {
    auction,
    product,
    auction_product,
  }
  .handle(input)
  .map(dispatcher::dispatch)?
  .map(AddAuctionProductCommand::apply)
  .map_err(|_| Error::NotCreated)?
  .ok_or(Error::NotCreated)
}

#[test]
fn test_add_auction_product() {
  let auction = Some(Auction {
    id: AuctionId::new(),
    show_id: ShowId::new(),
    ready_at: None,
    started_at: None,
    expired_at: None,
  });

  let product = Some(Product {
    id: ProductId::new(),
    name: Text::new("name"),
  });

  let input = AddAuctionProductInput {
    auction_id: auction.as_ref().unwrap().id,
    product_id: product.as_ref().unwrap().id,
  };

  let auction_product = Some(AuctionProduct {
    id: AuctionProductId::new(),
    auction_id: input.auction_id,
    product_id: input.product_id,
    best_bid_id: None,
  });

  let events = AddAuctionProductCommand {
    auction,
    product,
    auction_product,
  }
  .handle(input)
  .unwrap();

  assert_json_snapshot!(events, @"");
}
