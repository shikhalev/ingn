use super::*;
use chrono::{DateTime, Utc};
use std::fs;
use std::io;
use std::io::Write;
use std::ops;
use std::path;

pub struct File<'md, MD>
where
  MD: Meta<'md>,
{
  name: &'md str,
  path: &'md str,
  meta: &'md mut MD,
  writer: Option<fs::File>,
  reader: Option<io::BufReader<fs::File>>,
  owner: &'md dyn LinkedStorage<'md, Self, MD, fs::File>,
}

impl<'md, MD: Meta<'md>> File<'md, MD> {
  fn create(
    owner: &'md impl LinkedStorage<'md, Self, MD, fs::File>,
    name: &'md str,
    path: &'md str,
    meta: &'md mut MD,
  ) -> io::Result<Self> {
    Ok(Self {
      name,
      path,
      meta,
      writer: Some(fs::File::create(path)?),
      reader: None,
      owner,
    })
  }

  fn open_for_read(
    owner: &'md impl LinkedStorage<'md, Self, MD, fs::File>,
    name: &'md str,
    path: &'md str,
    meta: &'md mut MD,
  ) -> io::Result<Self> {
    Ok(Self {
      name,
      path,
      meta,
      writer: None,
      reader: Some(io::BufReader::new(fs::File::open(path)?)),
      owner,
    })
  }

  fn open_for_write(
    owner: &'md impl LinkedStorage<'md, Self, MD, fs::File>,
    name: &'md str,
    path: &'md str,
    meta: &'md mut MD,
  ) -> io::Result<Self> {
    Ok(Self {
      name,
      path,
      meta,
      writer: Some(fs::File::open(path)?),
      reader: None,
      owner,
    })
  }
}

impl<'md, MD: Meta<'md>> Drop for File<'md, MD> {
  fn drop(&mut self) {
    match self.flush() {
      _ => {}
    }
  }
}

impl<'md, MD: Meta<'md>> ops::Deref for File<'md, MD> {
  type Target = MD;

  fn deref(&self) -> &Self::Target {
    &self.meta
  }
}

impl<'md, MD: Meta<'md>> ops::DerefMut for File<'md, MD> {
  fn deref_mut(&mut self) -> &mut MD {
    &mut self.meta
  }
}

impl<'md, MD: Meta<'md>> io::Read for File<'md, MD> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match &mut self.reader {
      Some(r) => r.read(buf),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "File is open for write!",
      )),
    }
    // TODO: metadata?
  }
}

impl<'md, MD: Meta<'md>> io::BufRead for File<'md, MD> {
  fn fill_buf(&mut self) -> io::Result<&[u8]> {
    match &mut self.reader {
      Some(r) => r.fill_buf(),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "File is open for write!",
      )),
    }
  }

  fn consume(&mut self, size: usize) {
    match &mut self.reader {
      Some(r) => r.consume(size),
      None => unimplemented!(),
    }
  }
}

impl<'md, MD: Meta<'md>> io::Seek for File<'md, MD> {
  fn seek(&mut self, pos: std::io::SeekFrom) -> io::Result<u64> {
    match &mut self.reader {
      Some(r) => r.seek(pos),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "File is open for write!",
      )),
    }
  }
}

impl<'md, MD: Meta<'md>> io::Write for File<'md, MD> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match &mut self.writer {
      Some(w) => w.write(buf),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "File is open for read!",
      )),
    }
    // TODO: metadata?
  }

  fn flush(&mut self) -> io::Result<()> {
    match &mut self.writer {
      Some(w) => w.flush(),
      None => Ok(()), // do nothind - no error
    }
    // TODO: metadata
  }
}

impl<'md, MD: Meta<'md>> Data<'md, MD, fs::File> for File<'md, MD> {
  fn name(&self) -> &str {
    self.name
  }

  fn size(&self) -> io::Result<usize> {
    match &self.writer {
      Some(w) => Ok(w.metadata()?.len() as usize),
      None => Ok(fs::metadata(self.path)?.len() as usize),
    }
  }

  fn created(&self) -> io::Result<DateTime<Utc>> {
    Ok(fs::metadata(self.path)?.created()?.into())
  }

  fn modified(&self) -> io::Result<DateTime<Utc>> {
    Ok(fs::metadata(self.path)?.modified()?.into())
  }
}

impl<'md, MD: Meta<'md>> Linked for File<'md, MD> {
  fn parent(&self) -> io::Result<Option<String>> {
    self.owner.linked_parent(self.name)
  }

  fn children(&self) -> io::Result<Vec<String>> {
    self.owner.linked_children(self.name)
  }
}

pub struct Files {}
