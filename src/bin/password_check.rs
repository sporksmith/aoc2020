use aoc2020::passwords;

fn main() {
    use std::io::prelude::*;
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .filter(
                |line| passwords::check_line::<passwords::LegacyPasswordPolicy>(
                    line.as_ref().unwrap().as_str()
                )
                .unwrap()
            )
            .count()
    );
}
