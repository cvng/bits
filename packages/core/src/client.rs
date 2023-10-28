use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct UserCredential {
  pub user: String, // TODO: UserId,
}

#[derive(Default)]
pub struct Client {
  pub connection: DatabaseConnection,
  pub credential: Option<UserCredential>,
}

impl Client {
  pub fn new(connection: DatabaseConnection) -> Self {
    Self {
      connection,
      ..Default::default()
    }
  }

  pub fn connection(self, connection: DatabaseConnection) -> Self {
    Self { connection, ..self }
  }

  pub fn credential(self, credential: UserCredential) -> Self {
    Self {
      credential: Some(credential),
      ..self
    }
  }
}
