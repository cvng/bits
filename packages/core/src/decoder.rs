use jsonwebtoken::errors::Error;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct Claims {
  pub sub: String,
}

/// Get the `sub` from a token claims disabling the signature validation.
pub fn insecure_get_token_sub<T>(token: &str) -> Result<Option<T>, Error>
where
  T: std::str::FromStr,
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
