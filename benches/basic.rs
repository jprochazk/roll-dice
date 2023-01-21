use criterion::{black_box, criterion_group, criterion_main, Criterion};
use roll_dice::roll;

fn basic_usage(c: &mut Criterion) {
  c.bench_function("roll 1d20", |b| {
    b.iter(|| {
      let value = roll("1d20", 1423, u64::MAX).unwrap();
      black_box(value);
    })
  });
}

criterion_group!(benches, basic_usage);
criterion_main!(benches);
