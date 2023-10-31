use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;
use bits_core::bid::BidResult;
use bits_core::bid::BidInput;

pub struct BidMutation;

impl BidMutation {
  pub fn type_name() -> &'static str {
    "bid"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(BidResult::type_name()),
      move |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<BidInput>()?;

          let result = commands::bid::bid(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(BidInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(BidInput::to_input());
  builder.outputs.push(BidResult::to_object());
  builder.mutations.push(BidMutation::to_field());
  builder
}
