use async_graphql::scalar;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Text(fixedstr::str8);

scalar!(Text);
