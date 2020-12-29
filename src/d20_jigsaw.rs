use std::collections::{HashMap, HashSet};

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
        for id_and_tile_input in input.trim_end().split("\n\n") {
            //println!("Parsing '{}'", id_and_tile_input);
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

fn solve(
    ts: &TileSet,
    puzzle: &mut Puzzle,
    empty_positions: HashSet<Pos>,
) -> bool {
    let mut empty_positions = empty_positions;
    while !empty_positions.is_empty() {
        let empty_pos = *empty_positions.iter().next().unwrap();
        empty_positions.remove(&empty_pos);

        // If it's not actually empty, skip.
        if puzzle.get(&empty_pos).is_some() {
            continue;
        }

        // Find a tile that fits in this position.
        for (tid, tile) in &ts.tiles {
            // If this tile is already in use, skip.
            if puzzle.values().any(|(used_tid, _)| used_tid == tid) {
                continue;
            }

            // Try each orientation for this tile.
            for ori in &ORIENTATIONS {
                // If any neighbor doesn't match, skip.
                if let Some((neighbor_id, neighbor_ori)) =
                    puzzle.get(&(empty_pos.0 + 1, empty_pos.1))
                {
                    let neighbor = ts.tiles.get(neighbor_id).unwrap();
                    if neighbor.left(&neighbor_ori).reversed()
                        != tile.right(&ori)
                    {
                        continue;
                    }
                }
                if let Some((neighbor_id, neighbor_ori)) =
                    puzzle.get(&(empty_pos.0 - 1, empty_pos.1))
                {
                    let neighbor = ts.tiles.get(neighbor_id).unwrap();
                    if neighbor.right(&neighbor_ori).reversed()
                        != tile.left(&ori)
                    {
                        continue;
                    }
                }
                if let Some((neighbor_id, neighbor_ori)) =
                    puzzle.get(&(empty_pos.0, empty_pos.1 + 1))
                {
                    let neighbor = ts.tiles.get(neighbor_id).unwrap();
                    if neighbor.bottom(&neighbor_ori).reversed()
                        != tile.top(&ori)
                    {
                        continue;
                    }
                }
                if let Some((neighbor_id, neighbor_ori)) =
                    puzzle.get(&(empty_pos.0, empty_pos.1 - 1))
                {
                    let neighbor = ts.tiles.get(neighbor_id).unwrap();
                    if neighbor.top(&neighbor_ori).reversed()
                        != tile.bottom(&ori)
                    {
                        continue;
                    }
                }

                // Insert into puzzle; add new empty positions
                puzzle.insert(empty_pos, (*tid, *ori));

                let mut new_empty_pos = empty_positions.clone();
                // Push all neighboring positions. Don't bother to check if they're already
                // occupied - we check that when we use it.
                new_empty_pos.insert((empty_pos.0 + 1, empty_pos.1));
                new_empty_pos.insert((empty_pos.0 - 1, empty_pos.1));
                new_empty_pos.insert((empty_pos.0, empty_pos.1 + 1));
                new_empty_pos.insert((empty_pos.0, empty_pos.1 - 1));
                if solve(ts, puzzle, new_empty_pos) {
                    return true;
                }

                // Rest of the puzzle didn't work out with this placement. Undo and try the next.
                puzzle.remove(&empty_pos);
            }
        }
        // No tile fit this position; possibly because we're at an edge. Move to the next position.
    }

    // No more empty positions. We've finished the puzzle iff all tiles have been placed.
    puzzle.len() == ts.tiles.len()
}

pub fn part1(input: &str) -> u64 {
    let ts = TileSet::new(input);
    let mut puzzle = Puzzle::new();
    puzzle.insert((0, 0), (*ts.tiles.keys().next().unwrap(), (false, 0)));
    let mut empty_pos = HashSet::new();
    empty_pos.insert((1, 0));
    empty_pos.insert((-1, 0));
    empty_pos.insert((0, 1));
    empty_pos.insert((0, -1));
    if !solve(&ts, &mut puzzle, empty_pos) {
        panic!();
    }

    // Find edges
    let mut minx = i16::MAX;
    let mut miny = i16::MAX;
    let mut maxx = i16::MIN;
    let mut maxy = i16::MIN;
    for (x, y) in puzzle.keys() {
        minx = std::cmp::min(minx, *x);
        miny = std::cmp::min(miny, *y);
        maxx = std::cmp::max(maxx, *x);
        maxy = std::cmp::max(maxy, *y);
    }
    //println!("{:?}", puzzle);

    // Will panic if we ended up with a non-rectangular shape.
    [(minx, miny), (minx, maxy), (maxx, miny), (maxx, maxy)]
        .iter()
        .map(|pos| puzzle.get(pos).unwrap().0 as u64)
        .product()
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
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    assert_eq!(part1(input), 20899048083289);
}
