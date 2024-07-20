pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
  pub message: String,
}

impl Error {
  pub fn new(message: impl ToString) -> Self {
    Self {
      message: message.to_string(),
    }
  }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "error: {}", self.message)
  }
}

impl Error {
  pub fn overflow() -> Self {
    Self::new("integer overflow")
  }

  pub fn divide_by_zero() -> Self {
    Self::new("can't divide by zero")
  }

  pub fn roll_min_times() -> Self {
    Self::new("can't roll less than 0 times")
  }

  pub fn roll_min_sides() -> Self {
    Self::new("can't roll a less than 0-sided die")
  }

  pub fn too_many_rolls() -> Self {
    Self::new("too many rolls")
  }

  pub fn unexpected_eof() -> Self {
    Self::new("unexpected end of input")
  }

  pub fn unexpected_input(input: &str) -> Self {
    Self::new(format!("unexpected input: {input:?}"))
  }

  pub fn expected_input(expected: &str, got: &str) -> Self {
    Self::new(format!("expected {expected:?}, got {got:?}"))
  }

  pub fn invalid_integer(v: &str, e: impl std::fmt::Display) -> Self {
    Self::new(format!("invalid integer {v:?}: {e}"))
  }
}
