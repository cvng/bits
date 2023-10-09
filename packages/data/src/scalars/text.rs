use arrayvec::ArrayString;
use async_graphql::scalar;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Text(ArrayString<64>);

scalar!(Text, "String");
