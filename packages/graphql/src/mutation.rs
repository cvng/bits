use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use bits_core::commands;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
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
}
