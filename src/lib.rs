#[macro_use]
mod macros;
pub mod eval;
pub mod parse;
pub mod rng;

pub type Result<T, E = String> = std::result::Result<T, E>;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn roll(input: &str, seed: u64, limit: u64) -> Result<i64> {
  #[cfg(feature = "console_error_panic_hook")]
  {
    console_error_panic_hook::set_once();
  }

  let input = parse::parse(input)?;
  eval::eval(input, seed, limit)
}

#[cfg(test)]
mod tests;
