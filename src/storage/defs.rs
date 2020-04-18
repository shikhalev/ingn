use chrono::{DateTime, Utc};
use serde;
use std::hash;
use std::io;
use std::ops;

pub trait Meta<'md>:
  serde::Serialize + serde::Deserialize<'md> + Default + Clone + hash::Hash + Eq
{
}

pub trait Data<'md, MD, IO>:
  io::Write
  + io::BufRead
  + io::Seek
  + ops::Deref<Target = MD>
  + ops::DerefMut<Target = MD>
  + Drop
  + Sized
where
  MD: Meta<'md>,
  IO: io::Write + io::Read + io::Seek,
{
  fn name(&self) -> &str;

  fn size(&self) -> io::Result<usize>;
  fn created(&self) -> io::Result<DateTime<Utc>>;
  fn modified(&self) -> io::Result<DateTime<Utc>>;

  fn close(mut self) {
    match self.flush() {
      _ => {}
    }
  }
}

pub trait Linked {
  fn parent(&self) -> io::Result<Option<String>>;
  fn children(&self) -> io::Result<Vec<String>>;
}

pub trait Storage<'md, DT, MD, IO>
where
  DT: Data<'md, MD, IO>,
  MD: Meta<'md>,
  IO: io::Write + io::Read + io::Seek,
{
  fn create(&self, name: &str, proc: fn(&mut DT) -> io::Result<()>) -> io::Result<()>;
  fn read(&self, name: &str, proc: fn(&mut DT) -> io::Result<()>) -> io::Result<()>;
  fn update(&self, name: &str, proc: fn(&mut DT) -> io::Result<()>) -> io::Result<()>;
  fn delete(&self, name: &str) -> io::Result<()>;
  fn alias(&self, name: &str, src: &str) -> io::Result<()>;
}

pub trait LinkedStorage<'md, DT, MD, IO>: Storage<'md, DT, MD, IO>
where
  DT: Data<'md, MD, IO> + Linked,
  MD: Meta<'md>,
  IO: io::Write + io::Read + io::Seek,
{
  fn create_linked(&self, link: &str, proc: fn(&mut DT) -> io::Result<()>) -> io::Result<&str>;
  fn link(&self, name: &str, parent: &str) -> io::Result<()>;
  fn linked_parent(&self, name: &str) -> io::Result<Option<String>>;
  fn linked_children(&self, name: &str) -> io::Result<Vec<String>>;
}
