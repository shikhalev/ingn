use actix_service::ServiceFactory;
use actix_web::dev::Service;
use actix_web::error::Error;
use actix_web::{
  dev::{self, HttpServiceFactory},
  web::{self, HttpResponse},
  App, FromRequest, HttpServer, Result,
};
use dev::{ResourceDef, ServiceRequest, ServiceResponse};

use futures::future::{FutureExt, LocalBoxFuture, Ready};
use futures_util::future::ok;
use std::{collections::HashMap, task::Poll};

// #[derive(Serialize, Deserialize, Clone)]
// struct ImagePath {
//   filename: String,
// }

struct Img {
  path: String,
}

impl HttpServiceFactory for Img {
  fn register(self, cfg: &mut dev::AppService) {
    let rdef = ResourceDef::prefix(&self.path);
    cfg.register_service(rdef, None, self, None)
  }
}

impl ServiceFactory for Img {
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type Config = ();
  type Service = ImgService;
  type InitError = ();
  type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

  fn new_service(&self, _: Self::Config) -> Self::Future {
    let srv = ImgService {
      path: self.path.clone(),
    };
    ok(srv).boxed_local()
  }
}

struct ImgService {
  path: String,
}

impl Service for ImgService {
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type Future = Ready<Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Self::Request) -> Self::Future {
    let (rq, mut pl) = req.into_parts();
    let mut qu = web::Query::<HashMap<String, String>>::from_request(&rq, &mut pl)
      .into_inner()
      .unwrap()
      .into_inner();
    qu.insert("__path".to_string(), self.path.clone());
    qu.insert(
      "__path_info".to_string(),
      rq.match_info().path().to_string(),
    );
    ok(ServiceResponse::new(rq, HttpResponse::Ok().json(qu)))
  }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(Img {
      path: "/img/".to_string(),
    })
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
