use super::*;
use crate::utils;
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
  pub width: Size,
  pub height: Size,
  pub longer: Size,
  pub smaller: Size,
  #[serde(alias = "resize")]
  #[serde(alias = "mode")]
  pub resize_mode: ResizeMode,
  pub resize_up: bool,

  // crop block
  #[serde(alias = "anchor")]
  pub crop_anchor: Anchor,
  pub crop_x: Option<i32>,
  pub crop_y: Option<i32>,
  pub crop_width: Option<u32>,
  pub crop_height: Option<u32>,

  // frame block
  pub frame_width: Option<u32>,
  pub frame_height: Option<u32>,
  pub frame_border: Option<u32>,
  pub frame_color: Option<u32>,

  // custom block
  pub filters: utils::Names,

  // format
  pub format: Format,
}
