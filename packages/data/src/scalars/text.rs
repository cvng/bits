use arrayvec::ArrayString;
use async_graphql::scalar;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Text(ArrayString<64>);

impl Text {
  pub fn new(text: &str) -> Self {
    Self(ArrayString::from(text).unwrap())
  }
}

scalar!(Text, "String");
