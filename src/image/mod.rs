use actix_web::web;
use serde;
use serde::de;
use serde::ser;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

pub trait OR: Sized + Copy + PartialEq + Default {
    fn or<T: Into<Self>>(&self, other: T) -> Self {
        if *self == Self::default() {
            other.into()
        } else {
            *self
        }
    }
}

pub trait Merge: Sized {
    fn merge<T: Into<Self>>(&self, other: &T) -> &Self;
}

#[derive(Serialize, Deserialize)]
pub struct ImagePath {
    filename: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImageQuery {
    // #[serde(default)]
    pub width: Side,
    // #[serde(default)]
    pub height: Side,
    // #[serde(default)]
    format: Format,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Format {
    #[serde(alias = "jpeg")]
    JPEG,
    #[serde(alias = "png")]
    PNG,
    #[serde(alias = "gif")]
    GIF,
    #[serde(alias = "webp")]
    #[serde(alias = "WEBP")]
    WebP,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    Value(u32),
    Auto,
}

impl ser::Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Side::Value(x) => serializer.serialize_u32(*x),
            Side::Auto => serializer.serialize_str("Auto"),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct SideVisitor;
        impl<'de> serde::de::Visitor<'de> for SideVisitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Number or 'auto'")
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Side::Value(value as u32))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let val: String = value.to_string().to_lowercase().trim().to_string();
                match val.as_str() {
                    "auto" | "none" | "" => Ok(Side::Auto),
                    s => {
                        let v = s.parse::<u32>();
                        match v {
                            Ok(x) => Ok(Side::Value(x)),
                            Err(_) => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(SideVisitor)
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Auto
    }
}

impl OR for Side {}

impl Deref for Side {
    type Target = u32;

    fn deref(&self) -> &u32 {
        match self {
            Side::Value(x) => x,
            Side::Auto => panic!("No value!"),
        }
    }
}

impl From<u32> for Side {
    fn from(value: u32) -> Self {
        Self::Value(value)
    }
}

impl From<Option<u32>> for Side {
    fn from(value: Option<u32>) -> Side {
        match value {
            Some(x) => Side::Value(x),
            None => Side::Auto,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageInfo {
    name: String,
    title: Option<String>,
    width: Side,
    height: Side,
    format: Format,
}

pub trait Options {
    const WIDTH: u32;
    const HEIGHT: u32;
    const FORMAT: &'static str;
}

pub struct Defaults {}

impl Options for Defaults {
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 1080;
    const FORMAT: &'static str = "auto";
}

pub async fn get<Defs: Options>(
    path: web::Path<ImagePath>,
    query: web::Query<ImageQuery>,
) -> std::io::Result<web::Json<ImageInfo>> {
    let w = query.width;
    let f = query.format;
    let i = ImageInfo {
        name: path.filename.clone(),
        title: None,
        width: w,
        height: query.height,
        format: f,
    };
    Ok(web::Json(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_into() {
        let val: Side = 2u32.into();
        assert_eq!(val, Side::Value(2));
    }

    #[test]
    fn side_or() {
        let a = Side::Auto;
        let b = Side::Value(100);
        assert_eq!(b, a.or(b));
    }

    #[test]
    fn side_deref() {
        let a = Side::Value(2);
        assert_eq!(2, *a);
    }

    #[test]
    fn side_serial() {
        let s1 = serde_json::to_string(&Side::Value(1024)).unwrap();
        assert_eq!("1024", s1);
        let s2 = serde_json::to_string(&Side::Auto).unwrap();
        assert_eq!("\"Auto\"", s2);
        let v1 = serde_json::from_str("2048").unwrap();
        assert_eq!(Side::Value(2048), v1);
        let v2 = serde_json::from_str("\"none\"").unwrap();
        assert_eq!(Side::Auto, v2);
    }
}
