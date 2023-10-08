use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_show(
        &self,
        _ctx: &Context<'_>,
        input: bits_core::CreateShowInput,
    ) -> Result<bits_core::CreateShowPayload> {
        Ok(bits_core::create_show(input).await?)
    }
}
