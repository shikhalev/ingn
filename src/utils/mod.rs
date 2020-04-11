mod names;
#[doc(inline)]
pub use self::names::Names;

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

#[cfg(test)]
mod tests {

  #[test]
  fn option_or() {
    let a: Option<i32> = Some(42);
    let b: Option<i32> = None;
    assert_eq!(a, a.or(b));
    assert_eq!(a, b.or(a));
  }
}
