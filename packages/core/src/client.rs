use sea_orm::DatabaseConnection;

pub struct Client {
  connection: DatabaseConnection,
}

impl Client {
  pub fn new(connection: DatabaseConnection) -> Self {
    Self { connection }
  }

  pub fn connection(&self) -> &DatabaseConnection {
    &self.connection
  }
}
