use actix_files as fs;
// use actix_service::Service;
use actix_web::web;
use actix_web::{App, HttpServer};
use ingn::image;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route(
                "/img/{filename:.*}",
                web::get().to(image::get::<image::Defaults>),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
