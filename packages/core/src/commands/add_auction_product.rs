use crate::command::Command;
use crate::database;
use crate::dispatcher;
use async_graphql::dynamic::indexmap::IndexMap;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;
use bits_data::Auction;
use bits_data::AuctionId;
use bits_data::AuctionProduct;
use bits_data::AuctionProductId;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Utc;
use thiserror::Error;

pub struct AddAuctionProductInput {
  pub auction_id: AuctionId,
  pub product_id: ProductId,
}

impl AddAuctionProductInput {
  pub fn to_input_object() -> InputObject {
    InputObject::new("AddAuctionProductInput")
      .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("productId", TypeRef::named_nn(TypeRef::ID)))
  }
}

pub struct AddAuctionProductResult {
  pub auction_product: AuctionProduct,
}

impl AddAuctionProductResult {
  pub fn to_object() -> Object {
    Object::new("AddAuctionProductResult").field(Field::new(
      "id".to_string(),
      TypeRef::named_nn(TypeRef::ID),
      |ctx| {
        FieldFuture::new(
          async move { Ok(ctx.parent_value.as_value().cloned()) },
        )
      },
    ))
  }
}

impl From<AddAuctionProductResult> for Value {
  fn from(value: AddAuctionProductResult) -> Self {
    let mut map = IndexMap::new();
    map.insert(
      async_graphql::Name::new("id"),
      value.auction_product.id.to_string().into(),
    );
    Value::Object(map)
  }
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
      auction.ready_at = Some(auction_product.created_at);

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
    created_at: Utc::now(),
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
    id: "bbee6e9a-7985-461c-8ed6-6aa05084e335".parse().unwrap(),
    show_id: "048b47f4-3010-43ae-84c1-8088ab8488a8".parse().unwrap(),
    ready_at: None,
    started_at: None,
    expired_at: None,
  });

  let product = Some(Product {
    id: "2b1af787-2d94-4224-a2fc-1d8d155537c0".parse().unwrap(),
    name: "name".parse().unwrap(),
  });

  let input = AddAuctionProductInput {
    auction_id: auction.as_ref().unwrap().id,
    product_id: product.as_ref().unwrap().id,
  };

  let auction_product = Some(AuctionProduct {
    id: "177d1966-d688-486e-9b13-8709c0a434a0".parse().unwrap(),
    auction_id: input.auction_id,
    product_id: input.product_id,
    best_bid_id: None,
    created_at: "2023-10-17T02:55:11.787907Z".parse().unwrap(),
  });

  let events = AddAuctionProductCommand {
    auction,
    product,
    auction_product,
  }
  .handle(input)
  .unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "auction_product_created",
      "payload": {
        "auction_product": {
          "id": "177d1966-d688-486e-9b13-8709c0a434a0",
          "auction_id": "bbee6e9a-7985-461c-8ed6-6aa05084e335",
          "product_id": "2b1af787-2d94-4224-a2fc-1d8d155537c0",
          "best_bid_id": null,
          "created_at": "2023-10-17T02:55:11.787907Z"
        }
      }
    },
    {
      "type": "auction_marked_ready",
      "payload": {
        "auction": {
          "id": "bbee6e9a-7985-461c-8ed6-6aa05084e335",
          "show_id": "048b47f4-3010-43ae-84c1-8088ab8488a8",
          "ready_at": "2023-10-17T02:55:11.787907Z",
          "started_at": null,
          "expired_at": null
        }
      }
    }
  ]
  "###);
}
