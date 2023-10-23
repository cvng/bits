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
use async_graphql::Value;
use bits_core::commands;
use bits_core::AuctionProductId;
use bits_core::UserId;

pub struct Mutation;

impl Mutation {
  pub fn inputs() -> Vec<InputObject> {
    vec![
      InputObject::new("BidInput")
        .field(InputValue::new("userId", TypeRef::named_nn(TypeRef::ID)))
        .field(InputValue::new("productId", TypeRef::named_nn(TypeRef::ID)))
        .field(InputValue::new("amount", TypeRef::named_nn(TypeRef::INT))),
      InputObject::new("CommentInput")
        .field(InputValue::new("userId", TypeRef::named_nn(TypeRef::ID)))
        .field(InputValue::new("showId", TypeRef::named_nn(TypeRef::ID)))
        .field(InputValue::new("text", TypeRef::named_nn(TypeRef::STRING))),
      InputObject::new("CreateProductInput")
        .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING))),
      InputObject::new("CreateShowInput")
        .field(InputValue::new(
          "creator_id",
          TypeRef::named_nn(TypeRef::ID),
        ))
        .field(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING))),
      InputObject::new("StartShowInput")
        .field(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
      InputObject::new("AddAuctionProductInput")
        .field(InputValue::new("auctionId", TypeRef::named_nn(TypeRef::ID)))
        .field(InputValue::new("productId", TypeRef::named_nn(TypeRef::ID))),
    ]
  }

  pub fn mutations() -> Vec<Field> {
    vec![
      Field::new(
        "bid".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let input = ctx.args.get("input").unwrap().object().unwrap();

            let input = commands::bid::BidInput {
              user_id: input
                .get("userId")
                .unwrap()
                .string()
                .unwrap()
                .parse::<UserId>()
                .unwrap(),
              product_id: input
                .get("productId")
                .unwrap()
                .string()
                .unwrap()
                .parse::<AuctionProductId>()
                .unwrap(),
              amount: input.get("amount").unwrap().i64().unwrap(),
            };

            let result = bid(ctx.ctx, input).await?;

            Ok(Some(Value::from(result.ok)))
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
            Ok(Some(FieldValue::owned_any(comment(ctx).await?)))
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
            Ok(Some(FieldValue::owned_any(create_product(ctx).await?)))
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

  pub fn outputs() -> Vec<Object> {
    vec![
      Object::new("BidResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
      Object::new("CommentResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
      Object::new("CreateProductResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
      Object::new("CreateShowResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
      Object::new("StartShowResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
      Object::new("AddAuctionProductResult").field(Field::new(
        "ok".to_string(),
        TypeRef::named_nn(TypeRef::BOOLEAN),
        |_| {
          FieldFuture::new(async move { Ok(Some(FieldValue::owned_any(true))) })
        },
      )),
    ]
  }
}

pub struct MutationResult {
  pub ok: bool,
}

async fn bid(
  _ctx: &Context<'_>,
  _input: commands::bid::BidInput,
) -> Result<MutationResult> {
  Ok(MutationResult { ok: true })
}

async fn comment(
  _ctx: ResolverContext<'_>,
  // input: commands::comment::CommentInput,
) -> Result<commands::comment::CommentResult> {
  Err("comment".into()) // TODO: Ok(commands::comment::comment(input)?)
}

async fn create_product(
  _ctx: ResolverContext<'_>,
  // input: commands::create_product::CreateProductInput,
) -> Result<commands::create_product::CreateProductResult> {
  Err("create_product".into()) // TODO: Ok(commands::create_product::create_product(input).await?)
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
