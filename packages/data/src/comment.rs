use crate::ShowId;
use crate::UserId;
use async_graphql::SimpleObject;

pub type CommentId = crate::Id;

#[derive(Clone, SimpleObject)]
pub struct Comment {
    pub id: CommentId,
    pub author_id: UserId,
    pub show_id: ShowId,
}
