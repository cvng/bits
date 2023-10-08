use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use bits_ops as ops;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_show(
        &self,
        _ctx: &Context<'_>,
        input: ops::CreateShowInput,
    ) -> Result<ops::CreateShowPayload> {
        Ok(ops::create_show(input).await?)
    }
}
