use crate::Show;

pub enum Event {
  ShowCreated(ShowCreated),
  ShowStarted(ShowStarted),
}

pub struct ShowCreated {
  pub show: Show,
}

impl From<ShowCreated> for Event {
  fn from(event: ShowCreated) -> Self {
    Self::ShowCreated(event)
  }
}
pub struct ShowStarted {
  pub show: Show,
}

impl From<ShowStarted> for Event {
  fn from(event: ShowStarted) -> Self {
    Self::ShowStarted(event)
  }
}
