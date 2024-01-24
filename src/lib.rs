pub mod error;
pub mod eval;
pub mod parse;
pub mod rng;

pub use error::Result;

fn roll_impl(input: &str, seed: u64, limit: u64) -> Result<i64> {
  parse::parse(input)?.eval(seed, limit)
}

// Native bindings
#[cfg(not(target_arch = "wasm32"))]
pub fn roll(input: &str, seed: u64, limit: u64) -> Result<i64> {
  roll_impl(input, seed, limit)
}

// Wasm bindings
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn roll(input: &str, seed: u64, limit: u64) -> Result<i64, wasm_bindgen::JsError> {
  #[cfg(feature = "console_error_panic_hook")]
  {
    console_error_panic_hook::set_once();
  }

  roll_impl(input, seed, limit).map_err(|e| e.into())
}

#[cfg(test)]
mod tests;
