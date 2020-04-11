use crate::utils::OR;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Format {
  #[serde(alias = "jpeg")]
  JPEG,
  #[serde(alias = "png")]
  PNG,
  #[serde(alias = "gif")]
  GIF,
  // #[serde(alias = "webp")]
  // #[serde(alias = "WEBP")]
  // WebP,
  #[serde(alias = "auto")]
  #[serde(alias = "AUTO")]
  #[serde(alias = "None")]
  #[serde(alias = "NONE")]
  #[serde(alias = "none")]
  #[serde(alias = "")]
  Auto,
}

impl Default for Format {
  fn default() -> Self {
    Format::Auto
  }
}

impl OR for Format {}
