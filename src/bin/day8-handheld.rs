use aoc2020::handheld::*;

fn main() {
    let program = parse_program(std::io::stdin().lock());
    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "a" => acc_at_loop(&program),
        "b" => acc_after_fix(program),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
