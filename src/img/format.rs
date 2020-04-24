use serde_derive::{Deserialize, Serialize};
use std::path::Path;

/// Поддерживаемые форматы изображений.
///
/// `Format::Auto` в основном означает использование исходного формата.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Format {
  #[serde(alias = "jpeg")]
  #[serde(alias = "Jpeg")]
  #[serde(alias = "JPG")]
  #[serde(alias = "jpg")]
  #[serde(alias = "Jpg")]
  JPEG,
  #[serde(alias = "tiff")]
  #[serde(alias = "Tiff")]
  #[serde(alias = "TIF")]
  #[serde(alias = "tif")]
  #[serde(alias = "Tif")]
  TIFF,
  #[serde(alias = "gif")]
  #[serde(alias = "Gif")]
  GIF,
  #[serde(alias = "png")]
  #[serde(alias = "Png")]
  PNG,
  #[serde(alias = "auto")]
  #[serde(alias = "AUTO")]
  #[serde(alias = "None")]
  #[serde(alias = "NONE")]
  #[serde(alias = "none")]
  #[serde(alias = "")]
  Auto,
  Unknown,
}

impl Default for Format {
  fn default() -> Self {
    Format::Auto
  }
}

impl From<Option<Format>> for Format {
  fn from(value: Option<Format>) -> Self {
    match value {
      Some(v) => v,
      None => Format::Auto,
    }
  }
}

impl From<String> for Format {
  fn from(value: String) -> Self {
    match value.to_lowercase().as_str() {
      "jpeg" | "jpg" => Format::JPEG,
      "tiff" | "tif" => Format::TIFF,
      "gif" => Format::GIF,
      "png" => Format::PNG,
      "auto" | "none" | "" => Format::Auto,
      _ => Format::Unknown,
    }
  }
}

impl From<&str> for Format {
  fn from(value: &str) -> Self {
    match value.to_lowercase().as_str() {
      "jpeg" | "jpg" => Format::JPEG,
      "tiff" | "tif" => Format::TIFF,
      "gif" => Format::GIF,
      "png" => Format::PNG,
      "auto" | "none" | "" => Format::Auto,
      _ => Format::Unknown,
    }
  }
}

impl From<&Path> for Format {
  fn from(path: &Path) -> Self {
    match path.extension() {
      Some(os) => match os.to_str() {
        Some(s) => s.into(),
        None => Format::Unknown,
      },
      None => Format::Unknown,
    }
  }
}
