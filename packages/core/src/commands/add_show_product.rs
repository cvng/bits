use crate::database;
use crate::dispatch;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionMarkedReady;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::ShowProduct;
use bits_data::ShowProductAdded;
use bits_data::ShowProductId;
use bits_data::Utc;
use thiserror::Error;

#[derive(InputObject)]
pub struct AddShowProductInput {
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

#[derive(SimpleObject)]
pub struct AddShowProductPayload {
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

pub async fn add_show_product(
  input: AddShowProductInput,
) -> Result<AddShowProductPayload, Error> {
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

  let show_product = ShowProduct {
    id: ShowProductId::new(),
    auction_id: auction.id,
    product_id: product.id,
  };

  let mut events = vec![ShowProductAdded {
    id: show_product.id,
    auction_id: show_product.auction_id,
    product_id: show_product.product_id,
  }
  .into()];

  if auction.ready_at.is_none() {
    events.push(
      AuctionMarkedReady {
        id: auction.id,
        ready_at: Utc::now(),
      }
      .into(),
    )
  }

  dispatch::dispatch(events).ok();

  Ok(AddShowProductPayload { auction, product })
}
