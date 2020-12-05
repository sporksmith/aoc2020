use aoc2020::passport::{count_valid_passports, StrictPassport};

fn main() {
    println!(
        "{}",
        count_valid_passports::<StrictPassport, _>(std::io::stdin().lock())
            .unwrap()
    );
}
