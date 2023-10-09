use crate::id;
use crate::ShowId;
use crate::UserId;
use async_graphql::SimpleObject;

id!(CommentId);

#[derive(Clone, SimpleObject)]
#[graphql(name = "BaseComment")]
pub struct Comment {
  pub id: CommentId,
  pub author_id: UserId,
  pub show_id: ShowId,
}
