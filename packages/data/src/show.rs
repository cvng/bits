use crate::id;
use crate::DateTime;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(ShowId);

#[derive(Copy, Clone, Serialize, SimpleObject)]
pub struct Show {
  pub id: ShowId,
  pub created: Option<DateTime>,
  pub updated: Option<DateTime>,
  pub creator_id: UserId,
  pub name: Text,
  pub started: Option<DateTime>,
}
