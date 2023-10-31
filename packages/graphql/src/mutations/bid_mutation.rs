use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct BidMutation;

impl BidMutation {
  pub fn type_name() -> &'static str {
    "bid"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(commands::bid::BidResult::type_name()),
      move |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<commands::bid::BidInput>()?;

          let result = commands::bid::bid(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(commands::bid::BidInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(commands::bid::BidInput::to_input());
  builder.outputs.push(commands::bid::BidResult::to_object());
  builder.mutations.push(BidMutation::to_field());
  builder
}
