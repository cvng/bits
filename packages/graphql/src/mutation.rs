use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::ResolverContext;
use async_graphql::dynamic::TypeRef;
use async_graphql::Result;
use bits_core::commands;

pub struct Mutation;

impl Mutation {
  pub fn fields() -> Vec<Field> {
    vec![
      Field::new(
        "bid".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(bid(ctx).await?)))
          })
        },
      ),
      Field::new(
        "comment".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(comment(ctx).await?)))
          })
        },
      ),
      Field::new(
        "createProduct".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(create_product(ctx).await?)))
          })
        },
      ),
      Field::new(
        "createShow".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(create_show(ctx).await?)))
          })
        },
      ),
      Field::new(
        "startShow".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(start_show(ctx).await?)))
          })
        },
      ),
      Field::new(
        "addAuctionProduct".to_string(),
        TypeRef::named_nn("Boolean".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            Ok(Some(FieldValue::owned_any(add_auction_product(ctx).await?)))
          })
        },
      ),
    ]
  }
}

async fn bid(
  _ctx: ResolverContext<'_>,
  // input: commands::bid::BidInput,
) -> Result<commands::bid::BidResult> {
  Err("bid".into()) // TODO: Ok(commands::bid::bid(input)?)
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
