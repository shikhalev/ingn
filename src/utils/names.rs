use serde::de;
use serde::de::Error;
use serde_derive::Serialize;
use std::fmt;

#[derive(Serialize, Clone, Default, PartialEq, Debug)]
pub struct Names(Vec<String>);

impl<'de> serde::de::Deserialize<'de> for Names {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    struct NamesVisitor;

    impl<'de> serde::de::Visitor<'de> for NamesVisitor {
      type Value = Names;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("comma separated identifiers")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        if value.trim() == "" {
          Ok(Names(vec![]))
        } else {
          let ss: Vec<String> = value.split(",").map(|s| s.trim().to_string()).collect();
          Ok(Names(ss))
        }
      }

      fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
      where
        V: de::SeqAccess<'de>,
      {
        let mut value: Vec<String> = vec![];
        loop {
          match seq.next_element::<String>() {
            Ok(x) => match x {
              Some(y) => value.push(y.to_string()),
              None => break,
            },
            Err(_) => return Err(V::Error::invalid_value(de::Unexpected::Seq, &self)),
          }
        }
        Ok(Names(value))
      }
    }

    deserializer.deserialize_seq(NamesVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_urlencoded;
  use std::collections::HashMap;

  #[test]
  fn serial() {
    let v = Names(vec![
      "Alpha".to_string(),
      "Beta".to_string(),
      "Gamma".to_string(),
    ]);
    let a: HashMap<String, Names> = serde_urlencoded::from_str("Names=Alpha,Beta,Gamma").unwrap();
    assert_eq!(v, a["Names"]);
    let b: Names = serde_json::from_str("[\"Alpha\",\"Beta\",\"Gamma\"]").unwrap();
    assert_eq!(v, b);
  }
}
