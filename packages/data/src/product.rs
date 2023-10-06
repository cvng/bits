use async_graphql::SimpleObject;

pub type ProductId = crate::Id;

#[derive(Clone, SimpleObject)]
pub struct Product {
    pub id: ProductId,
}
