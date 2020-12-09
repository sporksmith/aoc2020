use aoc2020::*;

fn main() {
    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "7a" => bags::number_of_outer_bags_that_could_have_shiny(
            &bags::parse_input(std::io::stdin().lock()).unwrap(),
        ),
        "7b" => bags::number_of_bags_in_shiny(
            &bags::parse_input(std::io::stdin().lock()).unwrap(),
        ),
        "8a" => handheld::acc_at_loop(&handheld::parse_program(
            std::io::stdin().lock(),
        )) as usize,
        "8b" => handheld::acc_after_fix(handheld::parse_program(
            std::io::stdin().lock(),
        )) as usize,
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
