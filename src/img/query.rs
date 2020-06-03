use super::*;
use serde_derive::{Deserialize, Serialize};

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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Shift {
  XY(isize, isize),
  Auto(isize),
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Allow {
  #[serde(alias = "DENY")]
  #[serde(alias = "deny")]
  #[serde(alias = "No")]
  #[serde(alias = "NO")]
  #[serde(alias = "no")]
  #[serde(alias = "N")]
  #[serde(alias = "n")]
  #[serde(alias = "False")]
  #[serde(alias = "FALSE")]
  #[serde(alias = "false")]
  Deny,
  #[serde(alias = "ALLOW")]
  #[serde(alias = "allow")]
  #[serde(alias = "Yes")]
  #[serde(alias = "YES")]
  #[serde(alias = "yes")]
  #[serde(alias = "Y")]
  #[serde(alias = "y")]
  #[serde(alias = "True")]
  #[serde(alias = "TRUE")]
  #[serde(alias = "true")]
  Allow,
} // TODO: serialize to boolean

impl Default for Allow {
  fn default() -> Self {
    Allow::Deny
  }
}

trait OR: Sized + Copy + PartialEq + Default {
  fn or<T: Into<Self>>(&self, other: T) -> Self {
    if *self == Self::default() {
      other.into()
    } else {
      *self
    }
  }
}

impl<T: Sized + Copy + PartialEq + Default> OR for T {}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Query {
  // width: dim::Dim,
// height: dim::Dim,
// size: dim::Size,
// large: dim::Dim,

// mode: Mode,
// up: Allow,

// anchor: Anchor,

// x: Option<isize>,
// y: Option<isize>,
// shift: Option<Shift>,
// crop: Option<dim::Size>,

// format: Format,
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Specs {
  // width: usize,
// height: usize,
// mode: Mode,
// up: bool,

// anchor: Anchor,

// shift_x: Option<isize>,
// shift_y: Option<isize>,
// crop_width: Option<usize>,
// crop_height: Option<usize>,

// format: Format,
}

impl Specs {
  pub fn new(_: &Query, _: &Info) -> Self {
    // let mut w = query.width.or(query.size.width).or(query.large);
    // let mut h = query.height.or(query.size.height).or(query.large);
    Self::default()
  }
}
