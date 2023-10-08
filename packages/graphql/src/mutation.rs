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
    input: commands::CreateShowInput,
  ) -> Result<commands::CreateShowPayload> {
    Ok(commands::create_show(input).await?)
  }
}
