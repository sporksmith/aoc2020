use aoc2020::toboggan as t;
use std::io::Read;

fn main() {
    let right: usize = std::env::args()
        .nth(1)
        .expect("Missing right")
        .parse()
        .expect("Parsing right");
    let down: usize = std::env::args()
        .nth(2)
        .expect("Missing down")
        .parse()
        .expect("Parsing down");
    let mut map_string = String::new();
    std::io::stdin().read_to_string(&mut map_string).unwrap();
    let map: t::Map = map_string.parse().unwrap();
    println!("{}", t::trees_for_angle(&map, t::Angle { right, down }));
}
