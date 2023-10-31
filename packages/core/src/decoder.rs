use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use std::collections::HashSet;
use std::str::FromStr;

pub type Error = jsonwebtoken::errors::Error;

#[derive(Deserialize)]
pub struct Claims {
  pub sub: String,
}

/// Get the `sub` from a token claims without validating the signature.
pub fn insecure_get_token_sub<T>(token: &str) -> Result<Option<T>, Error>
where
  T: FromStr,
{
  let mut validation = Validation::new(Algorithm::HS256);
  validation.required_spec_claims = HashSet::new();
  validation.insecure_disable_signature_validation();

  jsonwebtoken::decode::<Claims>(
    token,
    &DecodingKey::from_secret(String::from("").as_ref()),
    &validation,
  )
  .map(|data| data.claims.sub.parse::<T>().ok())
}
