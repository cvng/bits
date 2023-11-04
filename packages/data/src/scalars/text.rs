use arrayvec::ArrayString;
use arrayvec::CapacityError;
use async_graphql::scalar;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Text(ArrayString<64>);

impl std::str::FromStr for Text {
  type Err = CapacityError<()>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Text(ArrayString::from(s).map_err(|err| err.simplify())?))
  }
}

scalar!(Text, "String");
