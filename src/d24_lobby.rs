use std::collections::HashSet;

type Coord = (i32, i32);

fn parse_coords(mut input: &str) -> Coord {
    let mut total_coord = (0, 0);
    let directions: Vec<_> = [
        ("w", (-1, 0)),
        ("e", (1, 0)),
        ("nw", (0, 1)),
        ("ne", (1, 1)),
        ("sw", (-1, -1)),
        ("se", (0, -1)),
    ]
    .iter()
    .collect();
    while !input.is_empty() {
        let (s, coord) = directions
            .iter()
            .find(|(s, _)| input.starts_with(s))
            .unwrap();
        input = input.strip_prefix(s).unwrap();
        total_coord.0 += coord.0;
        total_coord.1 += coord.1;
    }

    total_coord
}

pub fn part1(input: &str) -> usize {
    let mut floor = HashSet::<Coord>::new();
    for line in input.lines() {
        let coord = parse_coords(line);
        if !floor.insert(coord) {
            floor.remove(&coord);
        }
    }
    floor.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_coords() {
        assert_eq!(parse_coords("w"), (-1, 0));
        assert_eq!(parse_coords("wsw"), (-2, -1));
        assert_eq!(parse_coords("sww"), (-2, -1));
    }

    #[test]
    fn test_example() {
        let input = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(part1(input), 10);
    }
}
