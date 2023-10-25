use crate::command::Command;
use crate::dispatcher;
use async_graphql::dynamic::indexmap::IndexMap;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;
use bits_data::Event;
use bits_data::Product;
use bits_data::ProductId;
use bits_data::Text;
use thiserror::Error;

pub struct CreateProductInput {
  pub name: Text,
}

impl CreateProductInput {
  pub fn to_input_object() -> InputObject {
    InputObject::new("CreateProductInput")
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

pub struct CreateProductResult {
  pub product: Product,
}

impl CreateProductResult {
  pub fn to_object() -> Object {
    Object::new("CreateProductResult").field(Field::new(
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

impl From<CreateProductResult> for Value {
  fn from(value: CreateProductResult) -> Self {
    let mut map = IndexMap::new();
    map.insert(
      async_graphql::Name::new("id"),
      value.product.id.to_string().into(),
    );
    Value::Object(map)
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
    let product = self.product.ok_or(Error::NotCreated)?;

    Ok(vec![Event::product_created(product)])
  }

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result> {
    events.iter().fold(None, |_, event| match event {
      Event::ProductCreated { payload } => Some(CreateProductResult {
        product: payload.product,
      }),
      _ => None,
    })
  }
}

pub async fn create_product(
  input: CreateProductInput,
) -> Result<CreateProductResult, Error> {
  let product = Some(Product {
    id: ProductId::new(),
    name: input.name,
  });

  CreateProductCommand { product }
    .handle(input)
    .map(dispatcher::dispatch)?
    .map(CreateProductCommand::apply)
    .map_err(|_| Error::NotCreated)?
    .ok_or(Error::NotCreated)
}

#[test]
fn test_create_product() {
  let input = CreateProductInput {
    name: "name".parse().unwrap(),
  };

  let product = Some(Product {
    id: "f9f1436d-6ed5-4644-8e9e-7e14deffa2ec".parse().unwrap(),
    name: input.name,
  });

  let events = CreateProductCommand { product }.handle(input).unwrap();

  assert_json_snapshot!(events, @r###"
  [
    {
      "type": "product_created",
      "payload": {
        "product": {
          "id": "f9f1436d-6ed5-4644-8e9e-7e14deffa2ec",
          "name": "name"
        }
      }
    }
  ]
  "###);
}
