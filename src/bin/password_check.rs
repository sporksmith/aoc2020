use aoc2020::passwords;

fn main() {
    let policy_arg = std::env::args().nth(1).expect("missing policy");
    let check_line = match policy_arg.as_str() {
        "old" => passwords::check_line::<passwords::LegacyPasswordPolicy>,
        "new" => passwords::check_line::<passwords::NewPasswordPolicy>,
        _ => panic!("Unknown policy"),
    };
    use std::io::prelude::*;
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter(|line| check_line(line.as_ref().unwrap().as_str()).unwrap())
            .count()
    );
}
