use actix_files as af;
use actix_web::web;
use actix_web::{App, HttpServer};
use ingn::img;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(af::Files::new("/static", ".").show_files_listing())
      .route("/img/{filename:.*}", web::get().to(img::get))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
