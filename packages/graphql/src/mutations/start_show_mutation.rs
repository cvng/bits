use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct StartShowMutation;

impl StartShowMutation {
  pub fn type_name() -> &'static str {
    "startShow"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(commands::start_show::StartShowResult::type_name()),
      move |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<commands::start_show::StartShowInput>()?;

          let result = commands::start_show::start_show(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(commands::start_show::StartShowInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder
    .inputs
    .push(commands::start_show::StartShowInput::to_input());
  builder
    .outputs
    .push(commands::start_show::StartShowResult::to_object());
  builder.mutations.push(StartShowMutation::to_field());
  builder
}
