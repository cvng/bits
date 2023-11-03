use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::create_show::CreateShowInput;
use bits_core::create_show::CreateShowResult;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct CreateShowMutation;

impl CreateShowMutation {
  pub fn type_name() -> &'static str {
    "createShow"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(CreateShowResult::type_name()),
      |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<CreateShowInput>()?;

          let result =
            commands::create_show::create_show(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(CreateShowInput::type_name()),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder.inputs.push(CreateShowInput::to_input());
  builder.outputs.push(CreateShowResult::to_object());
  builder.mutations.push(CreateShowMutation::to_field());
  builder
}
