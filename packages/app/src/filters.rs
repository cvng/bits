use askama::Result;
use std::fmt::Display;

pub fn t(s: impl Display) -> Result<String> {
  Ok(rust_i18n::t!(&s.to_string()))
}

pub fn locale(_s: impl Display) -> Result<String> {
  Ok(rust_i18n::locale())
}
