#![forbid(unsafe_code)]

use askama::*;
use http::StatusCode;
use poem::IntoResponse;
use poem::Response;

pub fn into_response<T: Template>(t: &T) -> Response {
  match t.render() {
    Ok(body) => Response::builder()
      .header(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static(T::MIME_TYPE),
      )
      .body(body)
      .into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}
