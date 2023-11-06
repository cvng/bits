use askama::Result;
use std::fmt::Display;

pub trait Path: Display {}

impl Path for &String {}

impl Path for &str {}

pub fn t(s: impl Display) -> Result<String> {
  Ok(rust_i18n::t!(&s.to_string()))
}

pub fn locale(_s: impl Display) -> Result<String> {
  Ok(rust_i18n::locale())
}

pub fn path(s: impl Display, p: impl Path) -> Result<String> {
  match s.to_string().as_str() {
    "index" => Ok("/".to_string()),
    "get-started" => Ok("/get-started".to_string()),
    "show-detail" => Ok("/:name".replace(":name", p.to_string().as_str())),
    _ => todo!(),
  }
}
