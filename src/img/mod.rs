use actix_web::web;
use serde_derive::{Deserialize, Serialize};

mod size;
#[doc(inline)]
pub use self::size::Size;

mod format;
#[doc(inline)]
pub use self::format::Format;

mod query;
#[doc(inline)]
pub use self::query::Query;

#[derive(Serialize, Deserialize)]
pub struct ImagePath {
  filename: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct ImageQuery {
  width: Size,
  height: Size,
  side: Size,
  format: Format,
}

#[derive(Serialize, Deserialize)]
pub struct ImageInfo {
  name: String,
  title: Option<String>,
  width: Size,
  height: Size,
  format: Format,
}

pub trait Options {
  const WIDTH: u32;
  const HEIGHT: u32;
  const FORMAT: &'static str;
}

pub struct Defaults {}

impl Options for Defaults {
  const WIDTH: u32 = 1080;
  const HEIGHT: u32 = 1080;
  const FORMAT: &'static str = "auto";
}

pub async fn get<Defs: Options>(
  _path: web::Path<ImagePath>,
  query: web::Query<Query>,
) -> std::io::Result<web::Json<Query>> {
  let qq = query.clone();
  // qq.filters = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
  Ok(web::Json(qq))
}

//
