use sea_orm::DatabaseConnection;

pub struct Context {
  connection: DatabaseConnection,
}

impl Context {
  pub fn new(connection: DatabaseConnection) -> Self {
    Self { connection }
  }

  pub fn connection(&self) -> &DatabaseConnection {
    &self.connection
  }
}
