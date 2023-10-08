use crate::db;
use crate::events::ShowCreated;
use crate::Result;

pub fn show_created(event: ShowCreated) -> Result<()> {
    let ShowCreated { show } = event;

    db().shows.insert(show.id.clone(), show);

    Ok(())
}
