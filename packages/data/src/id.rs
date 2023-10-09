#[macro_export]
macro_rules! id {
  ($t:ident) => {
    #[derive(
      Clone,
      Copy,
      Eq,
      Hash,
      Ord,
      PartialEq,
      PartialOrd,
      Default,
      serde::Serialize,
      serde::Deserialize,
    )]
    pub struct $t(pub(crate) uuid::Uuid);

    impl $t {
      pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
      }
    }

    impl std::fmt::Display for $t {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
      }
    }

    impl std::str::FromStr for $t {
      type Err = uuid::Error;

      fn from_str(uuid_str: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(uuid_str)?))
      }
    }

    async_graphql::scalar!($t, "ID");
  };
}
