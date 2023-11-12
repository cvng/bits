use crate::dispatcher;
use crate::dispatcher::DispatchError;
use crate::Client;
use bits_data::Event;

/// Generic async command trait.
pub trait Command {
  type Error: From<DispatchError>;
  type Input: Clone;
  type Result;

  fn client(&self) -> &Client;

  async fn handle(&self, input: Self::Input)
    -> Result<Vec<Event>, Self::Error>;

  async fn apply(
    &self,
    input: Self::Input,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error>;

  async fn run(&self, input: Self::Input) -> Result<Self::Result, Self::Error> {
    let events = self.handle(input.clone()).await?;

    dispatcher::dispatch(self.client(), events.clone()).await?;

    self.apply(input, events).await
  }
}
