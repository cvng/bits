use crate::command::Command;
use crate::dispatcher::DispatchError;
use crate::Client;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_data::product;
use bits_data::sea_orm::EntityTrait;
use bits_data::Event;
use bits_data::PersonId;
use bits_data::Product;
use bits_data::ProductCreated;
use bits_data::ProductId;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductInput {
  pub creator_id: PersonId,
  pub name: String,
}

impl CreateProductInput {
  pub fn type_name() -> &'static str {
    "CreateProductInput"
  }

  pub fn to_input() -> InputObject {
    InputObject::new(Self::type_name())
      .field(InputValue::new("creatorId", TypeRef::named_nn(TypeRef::ID)))
      .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING)))
  }
}

#[derive(Clone, Serialize)]
pub struct CreateProductResult {
  pub product: Product,
}

impl CreateProductResult {
  pub fn type_name() -> &'static str {
    "CreateProductResult"
  }

  pub fn to_object() -> Object {
    Object::new(Self::type_name()).field(Field::new(
      "product",
      TypeRef::named_nn("Product"),
      |ctx| {
        FieldFuture::new(async move {
          Ok(Some(FieldValue::owned_any(
            ctx
              .parent_value
              .try_downcast_ref::<Self>()
              .cloned()
              .unwrap()
              .product,
          )))
        })
      },
    ))
  }
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("internal: db error")]
  Dx(#[from] DispatchError),
  #[error("product {0:?} not found")]
  NotFound(ProductId),
}

pub struct CreateProductCommand<'a> {
  pub client: &'a Client,
}

impl<'a> Command for CreateProductCommand<'a> {
  type Error = Error;
  type Input = CreateProductInput;
  type Result = CreateProductResult;

  fn client(&self) -> &Client {
    self.client
  }

  async fn handle(
    &self,
    input: Self::Input,
  ) -> Result<Vec<Event>, Self::Error> {
    Ok(vec![Event::ProductCreated {
      data: ProductCreated {
        id: ProductId::new_v4(),
        creator_id: input.creator_id,
        name: input.name,
      },
    }])
  }

  async fn apply(
    &self,
    _input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error> {
    let product_id = events
      .iter()
      .find_map(|event| match event {
        Event::ProductCreated { data, .. } => Some(data.id),
        _ => None,
      })
      .unwrap();

    let product = product::Entity::find_by_id(product_id)
      .one(&self.client.connection)
      .await
      .map_err(DispatchError::Database)?
      .ok_or(Error::NotFound(product_id))?;

    Ok(Self::Result { product })
  }
}

pub async fn create_product(
  client: &Client,
  input: CreateProductInput,
) -> Result<CreateProductResult, Error> {
  CreateProductCommand { client }.run(input).await
}
