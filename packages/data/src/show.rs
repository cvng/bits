use crate::id;
use crate::ProductId;
use crate::UserId;
use async_graphql::SimpleObject;

id!(ShowId);
id!(ShowProductId);

#[derive(Clone, Copy, SimpleObject)]
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
