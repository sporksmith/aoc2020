use aoc2020::toboggan as t;
use std::io::Read;

fn main() {
    let mut map_string = String::new();
    std::io::stdin().read_to_string(&mut map_string).unwrap();
    let map: t::Map = map_string.parse().unwrap();
    println!(
        "{}",
        t::trees_for_angle(&map, t::Angle { right: 3, down: 1 })
    );
}
