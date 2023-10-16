/// Generic command trait.
pub(crate) trait Command {
  type Error;
  type Event;
  type Input;
  type State;
  type Payload;

  fn handle(
    state: &Self::State,
    input: Self::Input,
  ) -> Result<Vec<Self::Event>, Self::Error>;

  fn apply(events: Vec<Self::Event>) -> Option<Self::Payload>;
}
