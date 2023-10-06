use async_graphql::SimpleObject;

pub type Id = u32;

#[derive(Clone, SimpleObject)]
pub struct Show {
    id: Id,
}
