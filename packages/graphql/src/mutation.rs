use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputObject;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use bits_core::commands;
use bits_core::Client;
use bits_core::UserCredential;

pub struct MutationBuilder {
  pub outputs: Vec<Object>,
  pub inputs: Vec<InputObject>,
  pub mutations: Vec<Field>,
}

impl MutationBuilder {
  pub fn new() -> Self {
    Self {
      outputs: Vec::new(),
      inputs: Vec::new(),
      mutations: Vec::new(),
    }
  }

  pub fn register(&mut self) {
    self.bid();
    self.comment();
    self.create_product();
    self.create_show();
    self.start_show();
    self.create_auction();
  }

  fn bid(&mut self) {
    self.outputs.push(commands::bid::BidResult::to_object());
    self.inputs.push(commands::bid::BidInput::to_input_object());
    self.mutations.push(
      Field::new(
        "bid".to_string(),
        TypeRef::named_nn("BidResult"),
        move |ctx| {
          FieldFuture::new(async move {
            let client = Client::default()
              .connection(ctx.data::<Client>()?.connection.clone())
              .credential(ctx.data::<UserCredential>()?.clone());

            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::bid::BidInput {
              auction_id: input
                .get("auctionId")
                .unwrap()
                .string()?
                .parse::<bits_core::AuctionId>()?,
              bidder_id: input
                .get("bidderId")
                .unwrap()
                .string()?
                .parse::<bits_core::UserId>()?,
              amount: input.get("amount").unwrap().i64()?.into(),
            };

            let result = commands::bid::bid(&client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("BidInput".to_string()),
      )),
    );
  }

  fn comment(&mut self) {
    self
      .outputs
      .push(commands::comment::CommentResult::to_object());
    self
      .inputs
      .push(commands::comment::CommentInput::to_input_object());
    self.mutations.push(
      Field::new(
        "comment".to_string(),
        TypeRef::named_nn("CommentResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let client = ctx.data::<Client>()?;

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
              text: input.get("text").unwrap().string()?.parse()?,
            };

            let result = commands::comment::comment(client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CommentInput".to_string()),
      )),
    )
  }

  fn create_product(&mut self) {
    self
      .outputs
      .push(commands::create_product::CreateProductResult::to_object());
    self
      .inputs
      .push(commands::create_product::CreateProductInput::to_input_object());
    self.mutations.push(
      Field::new(
        "createProduct".to_string(),
        TypeRef::named_nn("CreateProductResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let client = ctx.data::<Client>()?;

            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::create_product::CreateProductInput {
              creator_id: input
                .get("creatorId")
                .unwrap()
                .string()?
                .parse::<bits_core::UserId>()?,
              name: input.get("name").unwrap().string()?.parse()?,
            };

            let result =
              commands::create_product::create_product(client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CreateProductInput".to_string()),
      )),
    )
  }

  fn create_show(&mut self) {
    self
      .outputs
      .push(commands::create_show::CreateShowResult::to_object());
    self
      .inputs
      .push(commands::create_show::CreateShowInput::to_input_object());
    self.mutations.push(
      Field::new(
        "createShow".to_string(),
        TypeRef::named_nn("CreateShowResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let client = ctx.data::<Client>()?;

            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::create_show::CreateShowInput {
              creator_id: input
                .get("creatorId")
                .unwrap()
                .string()?
                .parse::<bits_core::UserId>()?,
              name: input.get("name").unwrap().string()?.parse()?,
            };

            let result =
              commands::create_show::create_show(client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CreateShowInput".to_string()),
      )),
    )
  }

  fn start_show(&mut self) {
    self
      .outputs
      .push(commands::start_show::StartShowResult::to_object());
    self
      .inputs
      .push(commands::start_show::StartShowInput::to_input_object());
    self.mutations.push(
      Field::new(
        "startShow".to_string(),
        TypeRef::named_nn("StartShowResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let client = ctx.data::<Client>()?;

            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::start_show::StartShowInput {
              id: input
                .get("id")
                .unwrap()
                .string()?
                .parse::<bits_core::ShowId>()?,
            };

            let result =
              commands::start_show::start_show(client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("StartShowInput".to_string()),
      )),
    )
  }

  fn create_auction(&mut self) {
    self
      .outputs
      .push(commands::create_auction::CreateAuctionResult::to_object());
    self
      .inputs
      .push(commands::create_auction::CreateAuctionInput::to_input_object());
    self.mutations.push(
      Field::new(
        "createAuction".to_string(),
        TypeRef::named_nn("CreateAuctionResult".to_string()),
        move |ctx| {
          FieldFuture::new(async move {
            let client = ctx.data::<Client>()?;

            let input = &ctx.args.get("input").unwrap().object()?;

            let input = commands::create_auction::CreateAuctionInput {
              show_id: input
                .get("showId")
                .unwrap()
                .string()?
                .parse::<bits_core::ShowId>()?,
              product_id: input
                .get("productId")
                .unwrap()
                .string()?
                .parse::<bits_core::ProductId>()?,
            };

            let result =
              commands::create_auction::create_auction(client, input).await?;

            Ok(Some(FieldValue::value(result)))
          })
        },
      )
      .argument(InputValue::new(
        "input".to_string(),
        TypeRef::named_nn("CreateAuctionInput".to_string()),
      )),
    )
  }
}
