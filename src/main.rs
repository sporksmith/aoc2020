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
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
