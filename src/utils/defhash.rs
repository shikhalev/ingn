use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait DefHash: Hash {
  fn default_hash(&self) -> String {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    format!("{:X}", hasher.finish())
  }
}

impl<T: Hash> DefHash for T {}
