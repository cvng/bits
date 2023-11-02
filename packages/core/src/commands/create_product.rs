use crate::command::Command;
use crate::dispatcher;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::UserId;
use thiserror::Error;

#[derive(Deserialize)]
pub struct CreateProductInput {
  #[serde(rename = "creatorId")]
  pub creator_id: UserId,
  pub name: String,
}

impl CreateProductInput {
  pub fn type_name() -> &'static str {
    "CreateProductInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

#[derive(Serialize)]
pub struct CreateProductResult {
  pub product: Product,
}

impl CreateProductResult {
  pub fn type_name() -> &'static str {
    "CreateProductResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
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

#[derive(Debug, Error)]
pub enum Error {
  #[error("product not created")]
  NotCreated,
  #[error("not found: {0}")]
  NotFound(ProductId),
}

pub struct CreateProductCommand {
  pub product: Option<Product>,
}

impl Command for CreateProductCommand {
  type Error = Error;
  type Event = Event;
  type Input = CreateProductInput;
  type Result = CreateProductResult;

  fn handle(
    &self,
    _input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    let product = self.product.clone().ok_or(Error::NotCreated)?;

    Ok(vec![Event::product_created(product)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ProductCreated { payload } => Some(CreateProductResult {
        product: payload.product.clone(),
      }),
      _ => None,
    })
  }
}

pub async fn create_product(
  client: &Client,
  input: CreateProductInput,
) -> Result<CreateProductResult, Error> {
  let product = Some(Product {
    id: ProductId::new_v4(),
    created: None,
    updated: None,
    creator_id: input.creator_id,
    name: input.name.clone(),
  });

  dispatcher::dispatch(client, CreateProductCommand { product }.handle(input)?)
    .await
    .map(CreateProductCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_create_product() {
  let input = CreateProductInput {
    creator_id: "abbba031-f122-42b8-b6ff-585ad245aadd".parse().unwrap(),
    name: "name".parse().unwrap(),
  };

  let product = Some(Product {
    id: "f9f1436d-6ed5-4644-8e9e-7e14deffa2ec".parse().unwrap(),
    created: None,
    updated: None,
    creator_id: input.creator_id,
    name: input.name.clone(),
  });

  let events = CreateProductCommand { product }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "product_created",
      "payload": {
        "product": {
          "id": "f9f1436d-6ed5-4644-8e9e-7e14deffa2ec",
          "created": null,
          "updated": null,
          "creator_id": "abbba031-f122-42b8-b6ff-585ad245aadd",
          "name": "name"
        }
      }
    }
  ]
  "###);
}
