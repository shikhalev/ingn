use actix_web::{web, Result};
use actix_web::{App, HttpServer};
use ingn::img;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ImagePath {
  filename: String,
}

pub async fn img_get(
  _path: web::Path<ImagePath>,
  query: web::Query<img::Query>,
) -> Result<web::Json<img::Query>> {
  let qq = query.clone();
  // qq.filters = crate::names![Alpha,Beta,Gamma];
  Ok(web::Json(qq))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().route("/img/{filename:.*}", web::get().to(img_get)))
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
