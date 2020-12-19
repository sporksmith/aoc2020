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
    {
        let input = std::fs::read_to_string("inputs/day10").unwrap();
        c.bench_function("10 parse", |b| b.iter(|| adapter::parse(&input)));

        let nums: Vec<_> = adapter::parse(&input);
        c.bench_function("10a", |b| b.iter(|| adapter::part1(&nums)));
        c.bench_function("10b", |b| b.iter(|| adapter::part2(&nums)));
    }
    {
        let input = std::fs::read_to_string("inputs/day11").unwrap();
        c.bench_function("11 parse", |b| b.iter(|| seating::parse(&input)));

        let grid = seating::parse(&input);
        c.bench_function("11a", |b| b.iter(|| seating::part1(&grid)));
        c.bench_function("11b", |b| b.iter(|| seating::part2(&grid)));
    }
    {
        let input = std::fs::read_to_string("inputs/day12").unwrap();
        c.bench_function("12 parse", |b| b.iter(|| d12_rain::parse(&input)));

        let input = d12_rain::parse(&input);
        c.bench_function("12a", |b| b.iter(|| d12_rain::part1(&input)));
        c.bench_function("12b", |b| b.iter(|| d12_rain::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day13").unwrap();
        c.bench_function("13a", |b| b.iter(|| d13_bus::part1(&input)));
        c.bench_function("13b", |b| b.iter(|| d13_bus::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day14").unwrap();
        c.bench_function("14a", |b| b.iter(|| d14_docking::part1(&input)));
        c.bench_function("14b", |b| b.iter(|| d14_docking::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day15").unwrap();
        c.bench_function("15a", |b| b.iter(|| d15_recitation::part1(&input)));
        //c.bench_function("15b", |b| b.iter(|| d15_recitation::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day16").unwrap();
        c.bench_function("16a", |b| b.iter(|| d16_ticket::part1(&input)));
        c.bench_function("16b", |b| b.iter(|| d16_ticket::part2(&input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
