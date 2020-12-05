use aoc2020::passport::count_valid_passports;

fn main() {
    println!(
        "{}",
        count_valid_passports(std::io::stdin().lock()).unwrap()
    );
}
