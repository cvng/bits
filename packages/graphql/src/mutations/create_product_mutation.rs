use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::create_product::CreateProductInput;
use bits_core::create_product::CreateProductResult;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct CreateProductMutation;

impl CreateProductMutation {
  pub fn type_name() -> &'static str {
    "createProduct"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(CreateProductResult::type_name()),
      move |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<CreateProductInput>()?;

          let result =
            commands::create_product::create_product(&client, input).await?;

          Ok(Some(to_value(result)?))
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
