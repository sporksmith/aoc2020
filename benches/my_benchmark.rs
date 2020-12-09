use aoc2020::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let input = std::fs::read_to_string("inputs/day7").unwrap();
        let rules =
            bags::parse_input(std::io::Cursor::new(input.as_bytes())).unwrap();
        c.bench_function("7a", |b| {
            b.iter(|| bags::number_of_outer_bags_that_could_have_shiny(&rules))
        });
        c.bench_function("7b", |b| {
            b.iter(|| bags::number_of_bags_in_shiny(&rules))
        });
    }
    {
        let input = std::fs::read_to_string("inputs/day8").unwrap();
        let program =
            handheld::parse_program(std::io::Cursor::new(input.as_bytes()));
        c.bench_function("8a", |b| b.iter(|| handheld::acc_at_loop(&program)));
        c.bench_function("8b", |b| {
            b.iter(|| handheld::acc_after_fix(program.clone()))
        });
    }
    {
        let input = std::fs::read_to_string("inputs/day9").unwrap();
        let nums: Vec<_> = encoding::parse(&input);
        c.bench_function("9 parse", |b| b.iter(|| encoding::parse(&input)));
        c.bench_function("9a", |b| b.iter(|| encoding::part1(&nums, 25)));
        c.bench_function("9b", |b| b.iter(|| encoding::part2(&nums, 25)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
