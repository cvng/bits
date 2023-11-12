use crate::dispatcher;
use crate::dispatcher::DispatchError;
use crate::Client;
use bits_data::Event;

/// Generic async command trait.
pub trait Command {
  type Error: From<DispatchError>;
  type Input;
  type Result;

  fn client(&self) -> &Client;
  fn input(&self) -> Self::Input;

  async fn handle(&self, input: Self::Input)
    -> Result<Vec<Event>, Self::Error>;

  async fn apply(
    &self,
    events: Vec<Event>,
  ) -> Result<Self::Result, Self::Error>;

  async fn run(&self) -> Result<Self::Result, Self::Error> {
    let events = self.handle(self.input()).await?;

    dispatcher::dispatch(self.client(), events.clone()).await?;

    self.apply(events).await
  }
}
