use aoc2020::find_sum_factors;

fn main() {
    use std::io::prelude::*;
    let target : u32 = std::env::args().nth(1).expect("missing target").parse().expect("parsing target");
    let xs : Vec<u32> = std::io::stdin().lock().lines().map(|s| s.unwrap().parse().expect("parsing line")).collect();
    let (x, y) = find_sum_factors(xs.as_slice(), target).unwrap();
    println!("{}", x*y);
}
