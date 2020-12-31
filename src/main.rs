use aoc2020::*;
use std::io::Cursor;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf).unwrap();

    let part = std::env::args().nth(1).expect("missing part");
    let res: Box<dyn std::fmt::Display> = match part.as_str() {
        "7a" => Box::new(bags::number_of_outer_bags_that_could_have_shiny(
            &bags::parse_input(Cursor::new(buf.as_bytes())).unwrap(),
        )),
        "7b" => Box::new(bags::number_of_bags_in_shiny(
            &bags::parse_input(Cursor::new(buf.as_bytes())).unwrap(),
        )),
        "8a" => Box::new(handheld::acc_at_loop(&handheld::parse_program(
            Cursor::new(buf.as_bytes()),
        ))),
        "8b" => Box::new(handheld::acc_after_fix(handheld::parse_program(
            Cursor::new(buf.as_bytes()),
        ))),
        "9a" => Box::new(encoding::part1(&encoding::parse(&buf), 25)),
        "9b" => Box::new(encoding::part2(&encoding::parse(&buf), 25)),
        "10a" => Box::new(adapter::part1(&adapter::parse(&buf))),
        "10b" => Box::new(adapter::part2(&adapter::parse(&buf))),
        "11a" => Box::new(seating::part1(&seating::parse(&buf))),
        "11b" => Box::new(seating::part2(&seating::parse(&buf))),
        "12a" => Box::new(d12_rain::part1(&d12_rain::parse(&buf))),
        "12b" => Box::new(d12_rain::part2(&d12_rain::parse(&buf))),
        "13a" => Box::new(d13_bus::part1(&buf)),
        "13b" => Box::new(d13_bus::part2(&buf)),
        "14a" => Box::new(d14_docking::part1(&buf)),
        "14b" => Box::new(d14_docking::part2(&buf)),
        "15a" => Box::new(d15_recitation::part1(&buf)),
        "15b" => Box::new(d15_recitation::part2(&buf)),
        "16a" => Box::new(d16_ticket::part1(&buf)),
        "16b" => Box::new(d16_ticket::part2(&buf)),
        "17a" => Box::new(d17_conway::part1(&buf)),
        "17b" => Box::new(d17_conway::part2(&buf)),
        "18a" => Box::new(d18_operation::part1(&buf)),
        "18b" => Box::new(d18_operation::part2(&buf)),
        "19a" => Box::new(d19_messages::part1(&buf)),
        "19b" => Box::new(d19_messages::part2(&buf)),
        "20a" => Box::new(d20_jigsaw::part1(&buf)),
        "20b" => Box::new(d20_jigsaw::part2(&buf)),
        "21a" => Box::new(d21_allergen::part1(&buf)),
        "21b" => Box::new(d21_allergen::part2(&buf)),
        "22a" => Box::new(d22_crab::part1(&buf)),
        "22b" => Box::new(d22_crab::part2(&buf)),
        "23a" => Box::new(d23_cups::part1(&buf)),
        "23b" => Box::new(d23_cups::part2(&buf)),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
