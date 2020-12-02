use aoc2020::find_sum_factors;

fn main() {
    use std::io::prelude::*;
    let target : i32 = std::env::args().nth(1).expect("missing target").parse().expect("parsing target");
    let xs : Vec<i32> = std::io::stdin().lock().lines().map(|s| s.unwrap().parse().expect("parsing line")).collect();
    let factors = find_sum_factors(2, xs.as_slice(), target).unwrap();
    println!("{}", factors.iter().fold(1, |x, y| x*y));
}
