use super::{Format, Info};

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Request {}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Mode {
  Fit,
  Fill,
  Exact,
}

impl Default for Mode {
  fn default() -> Self {
    Mode::Fit
  }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Anchor {
  Center,
  LT,
  RT,
  LB,
  RB,
  Auto,
}

impl Default for Anchor {
  fn default() -> Self {
    Anchor::Auto
  }
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Query {
  width: usize,
  height: usize,
  mode: Mode,

  anchor: Anchor,

  shift_x: Option<isize>,
  shift_y: Option<isize>,
  crop_width: Option<usize>,
  crop_height: Option<usize>,

  format: Format,
}

impl Query {
  pub fn new(query: &Request, info: &Info) -> Self {
    Self::default()
  }
}
