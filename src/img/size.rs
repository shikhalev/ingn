use crate::utils::OR;
use serde;
use serde::de;
use serde::ser;
use std::fmt;
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Size {
  Value(u32),
  Auto,
}

impl ser::Serialize for Size {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    match self {
      Size::Value(x) => serializer.serialize_u32(*x),
      Size::Auto => serializer.serialize_str("Auto"),
    }
  }
}

impl<'de> serde::de::Deserialize<'de> for Size {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    struct SizeVisitor;

    impl<'de> serde::de::Visitor<'de> for SizeVisitor {
      type Value = Size;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Number or 'auto'")
      }

      fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Size::Value(value as u32))
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        let val: String = value.to_string().to_lowercase().trim().to_string();
        match val.as_str() {
          "auto" | "none" | "" => Ok(Size::Auto),
          s => {
            let v = s.parse::<u32>();
            match v {
              Ok(x) => Ok(Size::Value(x)),
              Err(_) => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
            }
          }
        }
      }
    }

    deserializer.deserialize_any(SizeVisitor)
  }
}

impl Default for Size {
  fn default() -> Self {
    Size::Auto
  }
}

impl OR for Size {}

impl Deref for Size {
  type Target = u32;

  fn deref(&self) -> &u32 {
    match self {
      Size::Value(x) => x,
      Size::Auto => panic!("No value!"),
    }
  }
}

impl From<u32> for Size {
  fn from(value: u32) -> Self {
    Self::Value(value)
  }
}

impl From<Option<u32>> for Size {
  fn from(value: Option<u32>) -> Size {
    match value {
      Some(x) => Size::Value(x),
      None => Size::Auto,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn into() {
    let val: Size = 2u32.into();
    assert_eq!(val, Size::Value(2));
  }

  #[test]
  fn or() {
    let a = Size::Auto;
    let b = Size::Value(100);
    assert_eq!(b, a.or(b));
  }

  #[test]
  fn deref() {
    let a = Size::Value(2);
    assert_eq!(2, *a);
  }

  #[test]
  fn serial() {
    let s1 = serde_json::to_string(&Size::Value(1024)).unwrap();
    assert_eq!("1024", s1);
    let s2 = serde_json::to_string(&Size::Auto).unwrap();
    assert_eq!("\"Auto\"", s2);
    let v1 = serde_json::from_str("2048").unwrap();
    assert_eq!(Size::Value(2048), v1);
    let v2 = serde_json::from_str("\"none\"").unwrap();
    assert_eq!(Size::Auto, v2);
  }
}
