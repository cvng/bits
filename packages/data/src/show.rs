use crate::id;
use crate::DateTime;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(ShowId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
#[graphql(name = "BaseShow")]
pub struct Show {
  pub id: ShowId,
  pub creator_id: UserId,
  pub name: Text,
  pub started_at: Option<DateTime>,
}
