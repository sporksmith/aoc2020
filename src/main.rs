use aoc2020::*;
use std::io::Cursor;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf).unwrap();

    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "7a" => bags::number_of_outer_bags_that_could_have_shiny(
            &bags::parse_input(Cursor::new(buf.as_bytes())).unwrap(),
        ),
        "7b" => bags::number_of_bags_in_shiny(
            &bags::parse_input(Cursor::new(buf.as_bytes())).unwrap(),
        ),
        "8a" => handheld::acc_at_loop(&handheld::parse_program(Cursor::new(
            buf.as_bytes(),
        ))) as usize,
        "8b" => handheld::acc_after_fix(handheld::parse_program(Cursor::new(
            buf.as_bytes(),
        ))) as usize,
        "9a" => encoding::part1(&encoding::parse(&buf), 25) as usize,
        "9b" => encoding::part2(&encoding::parse(&buf), 25) as usize,
        "10a" => adapter::part1(&adapter::parse(&buf)) as usize,
        "10b" => adapter::part2(&adapter::parse(&buf)) as usize,
        "11a" => seating::part1(&seating::parse(&buf)) as usize,
        "11b" => seating::part2(&seating::parse(&buf)) as usize,
        "12a" => d12_rain::part1(&d12_rain::parse(&buf)) as usize,
        "12b" => d12_rain::part2(&d12_rain::parse(&buf)) as usize,
        "13a" => d13_bus::part1(&buf) as usize,
        "13b" => d13_bus::part2(&buf) as usize,
        "14a" => d14_docking::part1(&buf) as usize,
        "14b" => d14_docking::part2(&buf) as usize,
        "15a" => d15_recitation::part1(&buf) as usize,
        "15b" => d15_recitation::part2(&buf) as usize,
        "16a" => d16_ticket::part1(&buf) as usize,
        "16b" => d16_ticket::part2(&buf) as usize,
        "17a" => d17_conway::part1(&buf) as usize,
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
