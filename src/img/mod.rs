use actix_web::{web, Result};
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

mod engine;
#[doc(inline)]
pub use self::engine::Engine;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImagePath {
  filename: String,
}

pub async fn get(
  _path: web::Path<ImagePath>,
  query: web::Query<Query>,
) -> Result<web::Json<Query>> {
  let qq = query.clone();
  // qq.filters = crate::names![Alpha,Beta,Gamma];
  Ok(web::Json(qq))
}

//
