mod names;
#[doc(inline)]
pub use self::names::Names;

pub mod defhash;

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
