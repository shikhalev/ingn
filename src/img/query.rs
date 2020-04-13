use super::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum ResizeMode {
  Fit,
  Fill,
  Exact,
}

impl Default for ResizeMode {
  fn default() -> Self {
    ResizeMode::Fit
  }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Query {
  // resize block
  width: Size,
  height: Size,
  longer: Size,
  smaller: Size,
  #[serde(alias = "resize")]
  #[serde(alias = "mode")]
  resize_mode: ResizeMode,
  resize_up: bool,

  // crop block
  #[serde(alias = "anchor")]
  crop_anchor: Anchor,
  crop_x: Option<i32>,
  crop_y: Option<i32>,
  crop_width: Option<u32>,
  crop_height: Option<u32>,

  // format
  format: Format,
}
