use arrayvec::ArrayString;
use async_graphql::scalar;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Text(ArrayString<64>);

impl Text {
  pub fn new(text: &str) -> Self {
    Self(ArrayString::from(text).unwrap())
  }
}

scalar!(Text, "String");
