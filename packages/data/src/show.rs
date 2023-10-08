use crate::Id;
use crate::ProductId;
use crate::UserId;
use async_graphql::SimpleObject;

pub type ShowId = Id;

pub type ShowProductId = Id;

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseShow")]
pub struct Show {
  pub id: ShowId,
  pub creator_id: UserId,
}

#[derive(Clone, SimpleObject)]
pub struct ShowProduct {
  pub id: ShowProductId,
  pub show_id: ShowId,
  pub product_id: ProductId,
}
