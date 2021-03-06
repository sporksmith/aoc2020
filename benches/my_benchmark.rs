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
    {
        let input = std::fs::read_to_string("inputs/day17").unwrap();
        c.bench_function("17a", |b| b.iter(|| d17_conway::part1(&input)));
        c.bench_function("17b", |b| b.iter(|| d17_conway::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day18").unwrap();
        c.bench_function("18a_v0", |b| {
            b.iter(|| d18_operation::part1_v0(&input))
        });
        c.bench_function("18a", |b| b.iter(|| d18_operation::part1(&input)));
        c.bench_function("18b", |b| b.iter(|| d18_operation::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day19").unwrap();
        c.bench_function("19a_regex", |b| {
            b.iter(|| d19_messages::part1_regex(&input))
        });
        c.bench_function("19a", |b| b.iter(|| d19_messages::part1(&input)));
        c.bench_function("19b", |b| b.iter(|| d19_messages::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day20").unwrap();
        c.bench_function("20a", |b| b.iter(|| d20_jigsaw::part1(&input)));
        c.bench_function("20b", |b| b.iter(|| d20_jigsaw::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day21").unwrap();
        c.bench_function("21a", |b| b.iter(|| d21_allergen::part1(&input)));
        c.bench_function("21b", |b| b.iter(|| d21_allergen::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day22").unwrap();
        c.bench_function("22a", |b| b.iter(|| d22_crab::part1(&input)));
        c.bench_function("22b", |b| b.iter(|| d22_crab::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day23").unwrap();
        c.bench_function("23a", |b| b.iter(|| d23_cups::part1(&input)));
        c.bench_function("23b", |b| b.iter(|| d23_cups::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day24").unwrap();
        c.bench_function("24a", |b| b.iter(|| d24_lobby::part1(&input)));
        c.bench_function("24b", |b| b.iter(|| d24_lobby::part2(&input)));
    }
    {
        let input = std::fs::read_to_string("inputs/day25").unwrap();
        c.bench_function("25a", |b| b.iter(|| d25_combo::part1(&input)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
