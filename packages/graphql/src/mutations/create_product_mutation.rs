use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use bits_core::create_product;
use bits_core::create_product::CreateProductInput;
use bits_core::create_product::CreateProductResult;
use bits_core::data::seaography::Builder;
use bits_core::Client;
use bits_core::Token;

pub struct CreateProductMutation;

impl CreateProductMutation {
  pub fn type_name() -> &'static str {
    "createProduct"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(CreateProductResult::type_name()),
      |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(&ctx.data::<Client>()?.connection)
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<CreateProductInput>()?;

          let result = create_product::create_product(&client, input).await?;

          Ok(Some(FieldValue::owned_any(result)))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(CreateProductInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(CreateProductInput::to_input());
  builder.outputs.push(CreateProductResult::to_object());
  builder.mutations.push(CreateProductMutation::to_field());
  builder
}
