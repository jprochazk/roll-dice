use std::cell::RefCell;
use std::num::NonZeroU64;

use rand::rngs::SmallRng;
use rand::SeedableRng;

pub trait Rng {
  /// Returns a value in the range `0..max`
  fn range(&self, max: NonZeroU64) -> u64;
}

pub struct FakeRng {
  /// When set, the rng always returns `min(value, max-1)`,
  /// otherwise it returns the center of the range.
  value: Option<u64>,
}

pub fn fake(value: Option<u64>) -> FakeRng {
  FakeRng { value }
}

impl Rng for FakeRng {
  fn range(&self, max: NonZeroU64) -> u64 {
    let max = max.get();
    match self.value {
      Some(v) => std::cmp::min(v, max - 1),
      None => max / 2,
    }
  }
}

pub struct Prng {
  inner: RefCell<SmallRng>,
}

impl Rng for Prng {
  fn range(&self, max: NonZeroU64) -> u64 {
    use rand::Rng as _;

    self.inner.borrow_mut().gen_range(0..max.get())
  }
}

impl Prng {
  pub fn new(seed: u64) -> Self {
    Prng {
      inner: RefCell::new(SmallRng::seed_from_u64(seed)),
    }
  }
}
