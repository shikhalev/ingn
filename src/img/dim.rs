use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Dim {
  Value(usize),
  Auto,
}

impl Default for Dim {
  fn default() -> Self {
    Dim::Auto
  }
}

impl From<Option<usize>> for Dim {
  fn from(value: Option<usize>) -> Self {
    match value {
      Some(v) => Dim::Value(v),
      None => Dim::Auto,
    }
  }
}

impl From<usize> for Dim {
  fn from(value: usize) -> Self {
    Dim::Value(value)
  }
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Size {
  width: Dim,
  height: Dim,
}

