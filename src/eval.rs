use std::num::NonZeroU64;

use crate::error::{Error, Result};
use crate::rng::{Prng, Rng};

#[derive(Debug)]
pub struct Roll {
  pub ops: Vec<Op>,
  pub stack_size: usize,
}

impl Roll {
  pub fn eval(&self, seed: u64, limit: u64) -> Result<i64> {
    let rng = Prng::new(seed);
    self.eval_with_rng(limit, &rng)
  }

  #[allow(unused_assignments)]
  pub fn eval_with_rng(&self, limit: u64, rng: &dyn Rng) -> Result<i64> {
    let mut stack = vec![0i64; self.stack_size];
    let mut stack_top = 0;

    macro_rules! push {
      ($v:expr) => {{
        unsafe {
          let slot = stack.get_unchecked_mut(stack_top);
          *slot = $v;
          stack_top += 1;
        }
      }};
    }

    macro_rules! pop {
      () => {
        pop!(1)
      };
      (1) => {{
        unsafe {
          stack_top -= 1;
          (*stack.get_unchecked(stack_top + 0))
        }
      }};
      (2) => {{
        unsafe {
          stack_top -= 2;
          (
            *stack.get_unchecked(stack_top + 0),
            *stack.get_unchecked(stack_top + 1),
          )
        }
      }};
    }

    for op in &self.ops {
      match op {
        Op::Num(v) => {
          push!(*v);
        }
        Op::Add => {
          let (l, r) = pop!(2);
          let result = l.checked_add(r).ok_or_else(Error::overflow)?;
          push!(result);
        }
        Op::Sub => {
          let (l, r) = pop!(2);
          let result = l.checked_sub(r).ok_or_else(Error::overflow)?;
          push!(result);
        }
        Op::Mul => {
          let (l, r) = pop!(2);
          let result = l.checked_mul(r).ok_or_else(Error::overflow)?;
          push!(result);
        }
        Op::Div => {
          let (l, r) = pop!(2);
          if r == 0 {
            return Err(Error::new("can't divide by zero"));
          }
          let result = l / r;
          push!(result);
        }
        Op::Neg => {
          let v = pop!();
          let result = -v;
          push!(result);
        }
        Op::Dice => {
          let (times, sides) = pop!(2);
          if times < 0 {
            return Err(Error::roll_min_times());
          }
          let times = times as u64;
          if sides < 0 {
            return Err(Error::roll_min_sides());
          }
          let sides = sides as u64;
          if times > limit {
            return Err(Error::too_many_rolls());
          }
          let result = roll(times, sides, rng)?;
          push!(result);
        }
      }
    }

    Ok(pop!())
  }
}

fn roll(times: u64, sides: u64, rng: &dyn Rng) -> Result<i64> {
  if times == 0 {
    return Ok(0);
  }

  let Some(sides) = NonZeroU64::new(sides) else {
    return Ok(0);
  };

  let mut sum = 0i64;
  for _ in 0..times {
    let value: i64 = rng.range(sides).try_into().map_err(|_| Error::overflow())?;
    let value = value.checked_add(1i64).ok_or_else(Error::overflow)?;

    sum = sum.checked_add(value).ok_or_else(Error::overflow)?;
  }
  Ok(sum)
}

#[derive(Debug, PartialEq)]
pub enum Op {
  Num(i64),
  Add,
  Sub,
  Mul,
  Div,
  Neg,
  Dice,
}

impl From<i64> for Op {
  fn from(value: i64) -> Self {
    Op::Num(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rolls() {
    macro_rules! roll {
      (seed $seed:literal, $times:literal d $sides:literal = $result:literal) => {
        assert_eq!(
          roll($times, $sides, &$crate::rng::fake(Some($seed))),
          Ok($result),
          "failed to eval {}d{} with seed {}",
          $times,
          $sides,
          $seed,
        )
      };
    }

    roll!(seed 0, 0 d 0 = 0);
    roll!(seed 0, 1 d 0 = 0);
    roll!(seed 0, 0 d 1 = 0);

    roll!(seed 0, 1 d 1    = 1);
    roll!(seed 0, 1 d 2    = 1);

    roll!(seed 0,    1 d 1000 = 1);
    roll!(seed 1000, 1 d 1000 = 1000);

    roll!(seed 0,    2 d 1000 = 2);
    roll!(seed 1000, 2 d 1000 = 2000);

    roll!(seed 0,    100 d 5 = 100);
    roll!(seed 100,  100 d 5 = 500);
  }
}
