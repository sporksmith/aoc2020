use std::collections::HashMap;

type Orientation = (/* flipped: */ bool, /* rotations: */ u8);
type TileId = usize;
type Pos = (i16, i16);
type Puzzle = HashMap<Pos, (TileId, Orientation)>;

#[derive(Debug, Eq, PartialEq)]
struct TileSide(u16);

impl TileSide {
    fn reversed(&self) -> TileSide {
        TileSide(self.0.reverse_bits() >> 6)
    }
}

static ORIENTATIONS: [Orientation; 8] = [
    (false, 0),
    (false, 1),
    (false, 2),
    (false, 3),
    (true, 0),
    (true, 1),
    (true, 2),
    (true, 3),
];

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

struct Tile {
    bits: Vec<bool>,
}

impl Tile {
    fn new(input: &str) -> Tile {
        let mut bits = Vec::<bool>::new();
        for line in input.lines() {
            for c in line.chars() {
                assert!(c == '#' || c == '.');
                bits.push(c == '#')
            }
        }
        Tile { bits }
    }

    fn getbits(&self, indices: &[usize]) -> u16 {
        let mut res = 0;
        for i in indices {
            res *= 2;
            if self.bits[*i] {
                res += 1;
            }
        }
        res
    }

    fn top(&self, orientation: &Orientation) -> TileSide {
        let bits = match orientation {
            (false, 0) => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            (false, 1) => &[90, 80, 70, 60, 50, 40, 30, 20, 10, 0],
            (false, 2) => &[99, 98, 97, 96, 95, 94, 93, 92, 91, 90],
            (false, 3) => &[9, 19, 29, 39, 49, 59, 69, 79, 89, 99],
            (true, 0) => &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            (true, 1) => &[99, 89, 79, 69, 59, 49, 39, 29, 19, 9],
            (true, 2) => &[90, 91, 92, 93, 94, 95, 96, 97, 98, 99],
            (true, 3) => &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90],
            _ => panic!("whoops"),
        };
        TileSide(self.getbits(bits))
    }

    fn bottom(&self, orientation: &Orientation) -> TileSide {
        let (flipped, rot) = orientation;
        self.top(&(*flipped, (*rot + 2) % 4))
    }

    fn left(&self, orientation: &Orientation) -> TileSide {
        let (flipped, rot) = orientation;
        self.top(&(*flipped, (rot + 1) % 4))
    }

    fn right(&self, orientation: &Orientation) -> TileSide {
        let (flipped, rot) = orientation;
        self.top(&(*flipped, (rot + 3) % 4))
    }
}

struct TileSet {
    tiles: HashMap<TileId, Tile>,
}

impl TileSet {
    fn new(input: &str) -> TileSet {
        let mut tiles = HashMap::<TileId, Tile>::new();
        for id_and_tile_input in input.split("\n\n") {
            let mut tile_input_it = id_and_tile_input.splitn(2, '\n');
            let id_input = tile_input_it.next().unwrap();
            let tile_input = tile_input_it.next().unwrap();

            let id = id_input
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let tile = Tile::new(tile_input);
            tiles.insert(id, tile);
        }
        TileSet { tiles }
    }
}

#[cfg(test)]
#[test]
fn test_alignment() {
    let input = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";
    let ts = TileSet::new(input);
    assert_eq!(
        ts.tiles.get(&2311).unwrap().top(&(false, 0)).0,
        0b0011010010
    );
    assert_eq!(
        ts.tiles.get(&2311).unwrap().top(&(false, 1)).0,
        0b0100111110
    );

    let input = "\
Tile 1:
#...##.#..
..#.#..#.#
.###....#.
###.##.##.
.###.#####
.##.#....#
#...######
.....#..##
#.####...#
#.##...##.

Tile 2:
..###..###
###...#.#.
..#....#..
.#.#.#..##
##...#.###
##.##.###.
####.#...#
#...##..#.
##..#.....
..##.#..#.
";
    let ts = TileSet::new(input);
    assert_eq!(
        ts.tiles.get(&1).unwrap().right(&(false, 0)),
        ts.tiles.get(&2).unwrap().left(&(false, 0)).reversed()
    );
}
