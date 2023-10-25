use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::ResolverContext;
use async_graphql::dynamic::TypeRef;
use async_graphql::Context;
use async_graphql::Result;
use bits_core::commands;

pub struct Mutation;

impl Mutation {
  pub fn inputs() -> Vec<InputObject> {
    vec![
      commands::bid::BidInput::to_input_object(),
      commands::comment::CommentInput::to_input_object(),
      commands::create_product::CreateProductInput::to_input_object(),
      commands::create_show::CreateShowInput::to_input_object(),
      commands::start_show::StartShowInput::to_input_object(),
      commands::add_auction_product::AddAuctionProductInput::to_input_object(),
    ]
  }

  pub fn outputs() -> Vec<Object> {
    vec![
      commands::bid::BidResult::to_object(),
      commands::comment::CommentResult::to_object(),
      commands::create_product::CreateProductResult::to_object(),
      commands::create_show::CreateShowResult::to_object(),
      commands::start_show::StartShowResult::to_object(),
      commands::add_auction_product::AddAuctionProductResult::to_object(),
    ]
  }

  pub fn mutations() -> Vec<Field> {
    vec![
      Field::new(
        "bid".to_string(),
        TypeRef::named_nn("BidResult"),
        move |ctx| {
          FieldFuture::new(async move {
            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::bid::BidInput {
              user_id: input
                .get("userId")
                .unwrap()
                .string()?
                .parse::<bits_core::UserId>()?,
              product_id: input
                .get("productId")
                .unwrap()
                .string()?
                .parse::<bits_core::AuctionProductId>()?,
              amount: input.get("amount").unwrap().i64()?,
            };

            let result = Self::bid(ctx.ctx, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("BidInput".to_string()),
      )),
      Field::new(
        "comment".to_string(),
        TypeRef::named_nn("CommentResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::comment::CommentInput {
              user_id: input
                .get("userId")
                .unwrap()
                .string()?
                .parse::<bits_core::UserId>()?,
              show_id: input
                .get("showId")
                .unwrap()
                .string()?
                .parse::<bits_core::ShowId>()?,
              text: input
                .get("text")
                .unwrap()
                .string()?
                .parse::<bits_core::Text>()?,
            };

            let result = Self::comment(ctx.ctx, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CommentInput".to_string()),
      )),
      Field::new(
        "createProduct".to_string(),
        TypeRef::named_nn("CreateProductResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::create_product::CreateProductInput {
              name: input.get("name").unwrap().string()?.parse()?,
            };

            let result = Self::create_product(ctx.ctx, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CreateProductInput".to_string()),
      )),
      Field::new(
        "createShow".to_string(),
        TypeRef::named_nn("CreateShowResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(create_show(ctx).await?)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CreateShowInput".to_string()),
      )),
      Field::new(
        "startShow".to_string(),
        TypeRef::named_nn("StartShowResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(start_show(ctx).await?)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("StartShowInput".to_string()),
      )),
      Field::new(
        "addAuctionProduct".to_string(),
        TypeRef::named_nn("AddAuctionProductResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(add_auction_product(ctx).await?)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("AddAuctionProductInput".to_string()),
      )),
    ]
  }

  async fn bid(
    _ctx: &Context<'_>,
    input: commands::bid::BidInput,
  ) -> Result<commands::bid::BidResult> {
    Ok(commands::bid::bid(input)?)
  }

  async fn comment(
    _ctx: &Context<'_>,
    input: commands::comment::CommentInput,
  ) -> Result<commands::comment::CommentResult> {
    Ok(commands::comment::comment(input)?)
  }

  async fn create_product(
    _ctx: &Context<'_>,
    input: commands::create_product::CreateProductInput,
  ) -> Result<commands::create_product::CreateProductResult> {
    Ok(commands::create_product::create_product(input).await?)
  }
}

async fn create_show(
  _ctx: ResolverContext<'_>,
  // input: commands::create_show::CreateShowInput,
) -> Result<commands::create_show::CreateShowResult> {
  Err("create_show".into()) // TODO: (commands::create_show::create_show(input).await?)
}

async fn start_show(
  _ctx: ResolverContext<'_>,
  // input: commands::start_show::StartShowInput,
) -> Result<commands::start_show::StartShowResult> {
  Err("start_show".into()) // TODO: Ok(commands::start_show::start_show(input).await?)
}

async fn add_auction_product(
  _ctx: ResolverContext<'_>,
  // input: commands::add_auction_product::AddAuctionProductInput,
) -> Result<commands::add_auction_product::AddAuctionProductResult> {
  Err("add_auction_product".into()) // TODO: Ok(commands::add_auction_product::add_auction_product(input).await?)
}
