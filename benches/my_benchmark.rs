use aoc2020::handheld;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day8").unwrap();
    let program =
        handheld::parse_program(std::io::Cursor::new(input.as_bytes()));
    c.bench_function("8a", |b| b.iter(|| handheld::acc_at_loop(&program)));
    c.bench_function("8b", |b| {
        b.iter(|| handheld::acc_after_fix(program.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
