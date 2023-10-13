use crate::id;
use crate::ShowId;
use crate::Text;
use crate::UserId;
use async_graphql::SimpleObject;

id!(CommentId);

#[derive(Clone, Copy, Serialize, SimpleObject)]
#[graphql(name = "BaseComment")]
pub struct Comment {
  pub id: CommentId,
  pub user_id: UserId,
  pub show_id: ShowId,
  pub text: Text,
}
