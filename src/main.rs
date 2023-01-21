use std::sync::{Arc, Mutex};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use roll_dice::roll;

fn main() {
  let seeds = Arc::new(Mutex::new(vec![]));
  (0..1_000_000).into_par_iter().for_each({
    let seeds = seeds.clone();
    move |i| {
      if roll("(1d20)d(1d20)", i, u64::MAX).unwrap() == 0 {
        seeds.lock().unwrap().push(i);
      }
    }
  });

  let seeds: Vec<_> = std::mem::take(seeds.lock().unwrap().as_mut());
  println!("{seeds:?}");
}
