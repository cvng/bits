use crate::error::Error;
use bits_data::UserId;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use sea_orm::DatabaseConnection;
use std::collections::HashSet;

#[derive(Deserialize)]
struct Claims {
  sub: String,
}

#[derive(Clone)]
pub struct Token(pub String);

#[derive(Default)]
pub struct Client {
  pub connection: DatabaseConnection,
  pub token: Option<Token>,
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

  pub fn token(self, token: Token) -> Self {
    Self {
      token: Some(token),
      ..self
    }
  }
}

pub fn insecure_get_token_user(token: &Token) -> Result<UserId, Error> {
  let mut validation = Validation::new(Algorithm::HS256);

  validation.required_spec_claims = HashSet::new();
  validation.insecure_disable_signature_validation();

  let data = jsonwebtoken::decode::<Claims>(
    &token.0,
    &DecodingKey::from_secret("".as_ref()),
    &validation,
  )?;

  let user = data.claims.sub.parse::<UserId>()?;

  Ok(user)
}
