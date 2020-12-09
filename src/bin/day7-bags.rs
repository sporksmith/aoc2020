use aoc2020::bags::*;

fn main() {
    let rules = parse_input(std::io::stdin().lock()).unwrap();
    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "a" => number_of_outer_bags_that_could_have_shiny(&rules),
        "b" => number_of_bags_in_shiny(&rules),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
