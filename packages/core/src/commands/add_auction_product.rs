use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionMarkedReady;
use bits_data::AuctionProduct;
use bits_data::AuctionProductCreated;
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

pub async fn add_auction_product(
  input: AddAuctionProductInput,
) -> Result<AddAuctionProductPayload, Error> {
  let mut auction = database::db()
    .auctions
    .get(&input.auction_id)
    .cloned()
    .ok_or(Error::AuctionNotFound(input.auction_id))?;

  let product = database::db()
    .products
    .get(&input.product_id)
    .cloned()
    .ok_or(Error::ProductNotFound(input.product_id))?;

  let auction_product = AuctionProduct {
    id: AuctionProductId::new(),
    auction_id: auction.id,
    product_id: product.id,
    best_bid_id: None,
  };

  let mut events = vec![Event::AuctionProductCreated(AuctionProductCreated {
    auction_product,
  })];

  if auction.ready_at.is_none() {
    let ready_at = Utc::now();

    auction.ready_at = Some(ready_at);

    events.push(Event::AuctionMarkedReady(AuctionMarkedReady {
      id: auction.id,
      ready_at,
    }));
  }

  dispatch::dispatch(events).ok();

  Ok(AddAuctionProductPayload { auction, product })
}
