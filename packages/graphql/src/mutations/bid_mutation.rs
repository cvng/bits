use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use bits_core::bid;
use bits_core::bid::BidInput;
use bits_core::bid::BidResult;
use bits_core::seaography::Builder;
use bits_core::Client;

pub struct BidMutation;

impl BidMutation {
  pub fn type_name() -> &'static str {
    "bid"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(BidResult::type_name()),
      |ctx| {
        FieldFuture::new(async move {
          let client = ctx.data::<Client>()?;

          let input =
            ctx.args.get("input").unwrap().deserialize::<BidInput>()?;

          let result = bid::bid(client, input).await?;

          Ok(Some(FieldValue::owned_any(result)))
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
