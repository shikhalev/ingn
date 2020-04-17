use super::*;
use serde_derive::{Deserialize, Serialize};

// pub struct Source {}

// pub struct Variant {}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Info {
  pub title: Option<String>,
  pub size: Size,
  pub width: Size,
  pub height: Size,
  pub format: Format,
}
