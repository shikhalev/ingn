use super::*;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures::stream::Stream;
use std::io;

pub trait Getter {
  fn get(&self, filename: &str, query: &Specs) -> io::Result<Box<dyn Stream<Item = Bytes>>>;
}

#[derive(Copy, Clone)]
pub struct Info {
  pub size: usize,
  pub created: DateTime<Utc>,
  pub modified: DateTime<Utc>,
  pub width: usize,
  pub height: usize,
  pub format: Format,
}

pub trait Storage {
  fn create(&self, filename: &str, bytes: &Bytes) -> io::Result<()>;
  fn read(&self, filename: &str) -> io::Result<Bytes>;
  fn update(&self, filename: &str, bytes: &Bytes) -> io::Result<()>;
  fn delete(&self, filename: &str) -> io::Result<()>;
  fn info(&self, filename: &str) -> io::Result<Info>;
}
