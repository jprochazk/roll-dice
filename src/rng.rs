use std::cell::Cell;

pub trait Rng {
  fn range(&self, max: u64) -> u64;
}

pub struct FakeRng;

impl Rng for FakeRng {
  fn range(&self, max: u64) -> u64 {
    max / 2
  }
}

pub struct BasicRng(Cell<u64>);

impl Rng for BasicRng {
  fn range(&self, max: u64) -> u64 {
    // Adapted from: https://lemire.me/blog/2016/06/30/fast-random-shuffling/
    let mut r = self.u64();
    let mut hi = mul_high_u64(r, max);
    let mut lo = r.wrapping_mul(max);
    if lo < max {
      let t = max.wrapping_neg() % max;
      while lo < t {
        r = self.u64();
        hi = mul_high_u64(r, max);
        lo = r.wrapping_mul(max);
      }
    }
    hi
  }
}

impl BasicRng {
  pub fn new(seed: u64) -> Self {
    BasicRng(Cell::new(seed))
  }

  fn u64(&self) -> u64 {
    let s = self.0.get().wrapping_add(0xA0761D6478BD642F);
    self.0.set(s);
    let t = u128::from(s) * u128::from(s ^ 0xE7037ED1A0B428DB);
    (t as u64) ^ (t >> 64) as u64
  }
}

#[inline]
fn mul_high_u64(a: u64, b: u64) -> u64 {
  (((a as u128) * (b as u128)) >> 64) as u64
}
