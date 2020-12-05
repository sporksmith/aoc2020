use aoc2020::passport::{count_valid_passports, Passport};

fn main() {
    println!(
        "{}",
        count_valid_passports::<Passport, _>(std::io::stdin().lock()).unwrap()
    );
}
