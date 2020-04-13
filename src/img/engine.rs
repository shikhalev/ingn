use actix_web::{web, Result};
use std::path;

pub struct Engine {
  name: String,
  path: String,
  root: path::PathBuf,
  cache: path::PathBuf,
}

impl Engine {
  pub fn new<N: Into<String>, P: Into<String>, R: Into<path::PathBuf>, C: Into<path::PathBuf>>(
    name: N,
    path: P,
    root: R,
    cache: C,
  ) -> Self {
    Engine {
      name: name.into(),
      path: path.into(),
      root: root.into(),
      cache: cache.into(),
    }
  }

  pub async fn get() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }

  pub async fn head() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }
  pub async fn post() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }
  pub async fn put() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }
  pub async fn delete() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }
  pub async fn options() -> Result<web::HttpResponse> {
    Ok(web::HttpResponse::Ok().body(""))
  }
}

// macro img!(name, [path], [local], [cache])
// default path <- /name/
// default local <- /var/www/path
// default cache <- /var/tmp/img/path

#[macro_export]
macro_rules! img {
  ($name:ident, $path:expr, $root:expr, $cache:expr) => {
    mod $name {
      use actix_web::{web, Result};

      static engine: $crate::img::Engine =
        $crate::img::Engine::new(stringify!(name), $path, $root, $cache);

      pub async get() -> Result<web::HttpResponse> {
        engine.get().await;
      }
      //
    }
  };
  ($name:ident, $path:expr, $root:expr) => {
    img!($name, $path, $root, format!("/var/tmp/img/{}", $path))
  };
  ($name:ident, $path:expr) => {
    img!($name, $path, format!("/var/www/{}", $path))
  };
  ($name:ident) => {
    img!($name, format!("/{}/", stringify!($name)))
  };
}
