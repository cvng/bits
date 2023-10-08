use crate::Result;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bits_core::db;
use bits_core::dispatch;
use bits_core::Event;
use bits_core::ShowCreated;
use bits_data::Id;
use bits_data::Show;
use bits_data::Uuid;

#[derive(InputObject)]
pub struct CreateShowInput {
    pub creator_id: Id,
    pub name: String,
}

#[derive(SimpleObject)]
pub struct CreateShowPayload {
    pub show: Show,
}

pub async fn create_show(input: CreateShowInput) -> Result<CreateShowPayload> {
    let show = Show {
        id: Uuid::new_v4().into(),
        creator_id: input.creator_id,
        name: input.name,
    };

    dispatch(vec![Event::ShowCreated(ShowCreated { show: show.clone() })])?;

    Ok(CreateShowPayload {
        show: db().shows.get(&show.id).unwrap().clone(),
    })
}
