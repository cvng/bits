pub mod bid;
pub mod comment;
pub mod create_auction;
pub mod create_product;
pub mod create_show;

/// Generic command trait.
pub trait Command {
  type Error;
  type Event;
  type Input;
  type Result;

  fn handle(&self, input: Self::Input)
    -> Result<Vec<Self::Event>, Self::Error>;

  fn apply(events: Vec<Self::Event>) -> Option<Self::Result>;
}
