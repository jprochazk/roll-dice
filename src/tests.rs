use crate::error::Error;
use crate::eval::Op;
use crate::eval::Op::*;
use crate::parse::parse;
use crate::rng::FakeRng;

#[test]
fn parse_test() {
  macro_rules! ops {
    [$($op:expr),*] => { &[$($op.into()),*] };
  }

  #[track_caller]
  fn check(input: &str, ops: &[Op]) {
    assert_eq!(parse(input).unwrap().ops, ops)
  }

  #[track_caller]
  fn check_err(input: &str, err: &str) {
    assert_eq!(parse(input).unwrap_err(), Error::new(err))
  }

  check("10 + 5", ops![10, 5, Add]);
  check("10 - 5", ops![10, 5, Sub]);
  check("10 * 5", ops![10, 5, Mul]);
  check("10 / 5", ops![10, 5, Div]);
  check("10 d 5", ops![10, 5, Dice]);
  check("- 5", ops![5, Neg]);
  check("d 5", ops![1, 5, Dice]);
  check("10 d ( 50 + 50 )", ops![10, 50, 50, Add, Dice]);
  check("10 + 5 * 5", ops![10, 5, 5, Mul, Add]);
  check("10 * 5 + 5", ops![10, 5, Mul, 5, Add]);
  check_err("10 * *", "unexpected input: \"*\"");
  check_err("10 * (", "unexpected end of input");
  check_err("10 * (10", "expected \")\", got end of input");
  check_err("10 * asd", "unexpected input: \"as\"");
}

#[test]
fn eval_test() {
  #[track_caller]
  fn check(input: &str, result: i64) {
    assert_eq!(
      parse(input).and_then(|roll| roll.eval_with_rng(u64::MAX, &FakeRng)),
      Ok(result)
    )
  }

  #[track_caller]
  fn check_err(input: &str, err: &str, limit: u64) {
    assert_eq!(
      parse(input).and_then(|roll| roll.eval_with_rng(limit, &FakeRng)),
      Err(Error::new(err))
    )
  }

  check("10 + 5", 15);
  check("10 - 5", 5);
  check("10 * 5", 50);
  check("10 / 5", 2);
  check("10 d 5", 10 * 2);
  check("- 5", -5);
  check("d 5", 2);
  check("10 d ( 50 + 50 )", 10 * 50);
  check("10 + 5 * 5", 35);
  check("10 * 5 + 5", 55);
  check_err("(-1)d1", "can't roll less than 0 times", u64::MAX);
  check_err("10d(-1)", "can't roll a less than 0-sided die", u64::MAX);
  check_err("2d5", "too many rolls", 1);
  check_err("1/0", "can't divide by zero", u64::MAX);
}
