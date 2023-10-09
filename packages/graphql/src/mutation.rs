use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use bits_core::commands;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
  async fn create_product(
    &self,
    _ctx: &Context<'_>,
    input: commands::create_product::CreateProductInput,
  ) -> Result<commands::create_product::CreateProductPayload> {
    Ok(commands::create_product::create_product(input).await?)
  }

  async fn create_show(
    &self,
    _ctx: &Context<'_>,
    input: commands::create_show::CreateShowInput,
  ) -> Result<commands::create_show::CreateShowPayload> {
    Ok(commands::create_show::create_show(input).await?)
  }

  async fn start_show(
    &self,
    _ctx: &Context<'_>,
    input: commands::start_show::StartShowInput,
  ) -> Result<commands::start_show::StartShowPayload> {
    Ok(commands::start_show::start_show(input).await?)
  }

  async fn add_show_product(
    &self,
    _ctx: &Context<'_>,
    input: commands::add_show_product::AddShowProductInput,
  ) -> Result<commands::add_show_product::AddShowProductPayload> {
    Ok(commands::add_show_product::add_show_product(input).await?)
  }
}
