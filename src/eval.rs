use crate::rng::{BasicRng, Rng};
use crate::Result;

pub fn eval(input: Vec<Op>, seed: u64, limit: u64) -> Result<i64> {
  let rng = BasicRng::new(seed);
  eval_with_rng(input, limit, &rng)
}

pub fn eval_with_rng(input: Vec<Op>, limit: u64, rng: &dyn Rng) -> Result<i64> {
  let mut stack = Vec::new();
  for op in input {
    match op {
      Op::Num(v) => stack.push(v),
      Op::Add => {
        let (l, r) = pop2!(stack);
        stack.push(l + r);
      }
      Op::Sub => {
        let (l, r) = pop2!(stack);
        stack.push(l - r);
      }
      Op::Mul => {
        let (l, r) = pop2!(stack);
        stack.push(l * r);
      }
      Op::Div => {
        let (l, r) = pop2!(stack);
        stack.push(l / r);
      }
      Op::Neg => {
        let v = pop!(stack);
        stack.push(-v);
      }
      Op::Dice => {
        let (times, sides) = pop2!(stack);
        if times < 0 {
          return Err(format!("Weirdga can't roll less than 0 times"));
        }
        let times = times as u64;
        if sides < 0 {
          return Err(format!("Weirdga can't roll a less than 0-sided die"));
        }
        let sides = sides as u64;
        if times > limit {
          return Err(format!("Weirdga too many rolls"));
        }
        stack.push(roll(times, sides, rng)?);
      }
    }
  }
  Ok(pop!(stack))
}

fn roll(times: u64, sides: u64, rng: &dyn Rng) -> Result<i64> {
  let mut sum = 0;
  let mut i = times;
  while i > 0 {
    sum += rng.range(sides) as i64;
    i -= 1;
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
