use chrono::{DateTime, Utc};
use serde;
use std::hash;
use std::io;
use std::ops;

pub trait Meta<'md: 'prop, 'prop>:
  serde::Serialize + serde::Deserialize<'md> + Default + Clone + hash::Hash + Eq
{
  fn get<T>(&self, name: &'prop str) -> io::Result<&'md T>;
  fn get_mut<T>(&self, name: &'prop str) -> io::Result<&'md mut T>;
  fn set<T>(&self, name: &'prop str, value: impl Into<T>) -> io::Result<()>;
  fn reset<T>(&self, name: &'prop str) -> io::Result<()>;

  fn size(&self) -> Option<usize>;
  fn created(&self) -> Option<DateTime<Utc>>;
  fn modified(&self) -> Option<DateTime<Utc>>;
}

pub trait Data<'md: 'prop, 'prop, MD, IO>:
  io::Write
  + io::BufRead
  + io::Seek
  + ops::Deref<Target = MD>
  + ops::DerefMut<Target = MD>
  + Drop
  + Sized
where
  MD: Meta<'md, 'prop>,
  IO: io::Write + io::Read + io::Seek,
{
  fn name(&self) -> &'md str;

  fn close(mut self) -> io::Result<()> {
    self.flush()
  }
}

pub trait Storage<'md: 'idx + 'prop, 'idx, 'prop, DT, MD, IO>:
  ops::Index<&'idx str, Output = DT>
where
  DT: Data<'md, 'prop, MD, IO>,
  MD: Meta<'md, 'prop>,
  IO: io::Write + io::Read + io::Seek,
{
}
