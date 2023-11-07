use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use bits_core::seaography::Builder;
use bits_core::start;
use bits_core::start::StartInput;
use bits_core::start::StartResult;
use bits_core::Client;
use bits_core::Token;

pub struct StartMutation;

impl StartMutation {
  pub fn type_name() -> &'static str {
    "start"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(StartResult::type_name()),
      |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(&ctx.data::<Client>()?.connection)
            .token(ctx.data::<Token>()?.clone());

          let input =
            ctx.args.get("input").unwrap().deserialize::<StartInput>()?;

          let result = start::start(&client, input).await?;

          Ok(Some(FieldValue::owned_any(result)))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(StartInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(StartInput::to_input());
  builder.outputs.push(StartResult::to_object());
  builder.mutations.push(StartMutation::to_field());
  builder
}
