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
