use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use async_graphql::to_value;
use bits_core::commands;
use bits_core::Client;
use bits_core::Token;
use seaography::Builder;

pub struct CreateAuctionMutation;

impl CreateAuctionMutation {
  pub fn type_name() -> &'static str {
    "createAuction"
  }

  pub fn to_field() -> Field {
    Field::new(
      Self::type_name(),
      TypeRef::named_nn(
        commands::create_auction::CreateAuctionResult::type_name(),
      ),
      move |ctx| {
        FieldFuture::new(async move {
          let client = Client::default()
            .connection(ctx.data::<Client>()?.connection.clone())
            .token(ctx.data::<Token>()?.clone());

          let input = ctx
            .args
            .get("input")
            .unwrap()
            .deserialize::<commands::create_auction::CreateAuctionInput>(
          )?;

          let result =
            commands::create_auction::create_auction(&client, input).await?;

          Ok(Some(to_value(result)?))
        })
      },
    )
    .argument(InputValue::new(
      "input",
      TypeRef::named_nn(
        commands::create_auction::CreateAuctionInput::type_name(),
      ),
    ))
  }
}

pub fn register(mut builder: Builder) -> Builder {
  builder
    .inputs
    .push(commands::create_auction::CreateAuctionInput::to_input());
  builder
    .outputs
    .push(commands::create_auction::CreateAuctionResult::to_object());
  builder.mutations.push(CreateAuctionMutation::to_field());
  builder
}
