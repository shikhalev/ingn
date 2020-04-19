use super::*;
use chrono::{DateTime, Utc};
use std::fs;
use std::io;
use std::io::Write;
use std::ops;
use std::path;

pub struct File<MD>
where
  MD: Meta,
{
  name: String,
  path: path::PathBuf,
  meta: MD,
  writer: Option<fs::File>,
  reader: Option<io::BufReader<fs::File>>,
  extension: Option<String>,
}

impl<MD: Meta> File<MD> {
  fn create(name: &str, path: &path::Path, meta: &MD) -> io::Result<Self> {
    Ok(Self {
      name: name.to_owned(),
      path: path.to_owned(),
      meta: meta.clone(),
      writer: Some(fs::File::create(path)?),
      reader: None,
      extension: match path.extension() {
        Some(os) => match os.to_str() {
          Some(s) => Some(s.to_lowercase()),
          None => None,
        },
        None => None,
      },
    })
  }

  fn open_for_read(
    // owner: &impl LinkedStorage<Self, MD, fs::File>,
    name: &str,
    path: &path::Path,
    meta: &MD,
  ) -> io::Result<Self> {
    Ok(Self {
      name: name.to_owned(),
      path: path.to_owned(),
      meta: meta.clone(),
      writer: None,
      reader: Some(io::BufReader::new(fs::File::open(path)?)),
      extension: match path.extension() {
        Some(os) => match os.to_str() {
          Some(s) => Some(s.to_lowercase()),
          None => None,
        },
        None => None,
      },
    })
  }

  fn open_for_write(
    // owner: &impl LinkedStorage<Self, MD, fs::File>,
    name: &str,
    path: &path::Path,
    meta: &MD,
  ) -> io::Result<Self> {
    Ok(Self {
      name: name.to_owned(),
      path: path.to_owned(),
      meta: meta.clone(),
      writer: Some(fs::File::open(path)?),
      reader: None,
      extension: match path.extension() {
        Some(os) => match os.to_str() {
          Some(s) => Some(s.to_lowercase()),
          None => None,
        },
        None => None,
      },
    })
  }
}

impl<MD: Meta> Drop for File<MD> {
  fn drop(&mut self) {
    match self.flush() {
      _ => {}
    }
  }
}

impl<MD: Meta> ops::Deref for File<MD> {
  type Target = MD;

  fn deref(&self) -> &Self::Target {
    &self.meta
  }
}

impl<MD: Meta> ops::DerefMut for File<MD> {
  fn deref_mut(&mut self) -> &mut MD {
    &mut self.meta
  }
}

impl<MD: Meta> io::Read for File<MD> {
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

impl<MD: Meta> io::BufRead for File<MD> {
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

impl<MD: Meta> io::Seek for File<MD> {
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

impl<MD: Meta> io::Write for File<MD> {
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

impl<MD: Meta> Data<MD, fs::File> for File<MD> {
  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn size(&self) -> io::Result<usize> {
    match &self.writer {
      Some(w) => Ok(w.metadata()?.len() as usize),
      None => Ok(fs::metadata(self.path.as_path())?.len() as usize),
    }
  }

  fn created(&self) -> io::Result<DateTime<Utc>> {
    Ok(fs::metadata(self.path.as_path())?.created()?.into())
  }

  fn modified(&self) -> io::Result<DateTime<Utc>> {
    Ok(fs::metadata(self.path.as_path())?.modified()?.into())
  }
}

impl<MD: Meta> Linked for File<MD> {
  fn parent(&self) -> io::Result<Option<String>> {
    unimplemented!()
  }

  fn children(&self) -> io::Result<Vec<String>> {
    unimplemented!()
  }
}

pub struct Files {
  root: String,
}

impl Files {
  pub fn new(root: &str) -> Self {
    Self {
      root: root.to_owned(),
    }
  }

  #[inline]
  fn expand_path(&self, path: &str) -> io::Result<path::PathBuf> {
    fs::canonicalize(format!("{}/{}", self.root, path))
  }
}

impl<MD> Storage<File<MD>, MD, fs::File> for Files
where
  MD: Meta,
{
  fn create(
    &self,
    name: &str,
    meta: &MD,
    proc: fn(&mut File<MD>) -> io::Result<()>,
  ) -> io::Result<()> {
    // TODO: metadata  processing
    let mut data = File::create(name, &self.expand_path(name)?, meta)?;
    proc(&mut data)?;
    data.close();
    Ok(())
  }

  fn read(&self, name: &str, proc: fn(&mut File<MD>) -> io::Result<()>) -> io::Result<()> {
    // TODO: metadata  processing
    let mut data = File::open_for_read(name, &self.expand_path(name)?, &MD::default())?;
    proc(&mut data)?;
    data.close();
    Ok(())
  }

  fn update(
    &self,
    name: &str,
    meta: &MD,
    proc: fn(&mut File<MD>) -> io::Result<()>,
  ) -> io::Result<()> {
    // TODO: metadata and links processing
    let mut data = File::open_for_write(name, &self.expand_path(name)?, meta)?;
    proc(&mut data)?;
    data.close();
    Ok(())
  }

  fn delete(&self, name: &str) -> io::Result<()> {
    unimplemented!()
  }
  fn alias(&self, name: &str, src: &str) -> io::Result<()> {
    unimplemented!()
  }
}

impl<MD> LinkedStorage<File<MD>, MD, fs::File> for Files
where
  MD: Meta,
{
  fn create_linked(
    &self,
    link: &str,
    meta: &MD,
    proc: fn(&mut File<MD>) -> io::Result<()>,
  ) -> io::Result<&str> {
    unimplemented!()
  }
  fn linked_parent(&self, name: &str) -> io::Result<Option<String>> {
    unimplemented!()
  }
  fn linked_children(&self, name: &str) -> io::Result<Vec<String>> {
    unimplemented!()
  }
}
