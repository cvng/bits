use async_graphql::SimpleObject;

pub type ProductId = crate::Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseProduct")]
pub struct Product {
    pub id: ProductId,
}
