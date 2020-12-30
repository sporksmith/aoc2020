use ndarray::{s, Array, Array2};
use std::collections::{HashMap, HashSet};

type Orientation = (/* flipped: */ bool, /* rotations: */ u8);
type TileId = usize;
type Pos = (i16, i16);
type Puzzle = HashMap<Pos, (TileId, Tile)>;

#[derive(Debug, Eq, PartialEq)]
struct TileSide(Vec<i8>);

impl TileSide {
    fn reversed(&self) -> TileSide {
        let mut v = self.0.clone();
        v.reverse();
        TileSide(v)
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

#[derive(Clone)]
struct Tile {
    bits: Array2<i8>,
}

impl Tile {
    fn new(input: &str) -> Tile {
        let mut v = Vec::<i8>::new();
        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            //println!("Processing line {} of length {}", height, line.len());
            width = line.len();
            height += 1;
            for c in line.chars() {
                v.push(if c == '#' { 1 } else { 0 });
            }
        }
        //println!("height {} width {} arr.len {} str.len {}", height, width, v.len(), input.len());
        Tile {
            bits: Array::from_shape_vec((height, width), v).unwrap(),
        }
    }

    fn monster() -> Tile {
        Tile::new(
            "\
.                 # 
#    ##    ##    ###
 #  #  #  #  #  #   ",
        )
    }

    fn contains(&self, other: &Tile, pos: &Pos) -> bool {
        for y in 0..other.bits.dim().0 {
            for x in 0..other.bits.dim().1 {
                if other.bits[(y, x)] == 1
                    && self.bits.get((y + pos.1 as usize, x + pos.0 as usize))
                        != Some(&1)
                {
                    return false;
                }
            }
        }
        true
    }

    fn transformed(&self, ori: &Orientation) -> Tile {
        assert_eq!(self.bits.dim().0, self.bits.dim().1);
        let dim = self.bits.dim().0;

        let mut bits = if ori.0 {
            let mut flipped = Array2::<i8>::zeros(self.bits.dim());
            for x in 0..dim {
                for y in 0..dim {
                    flipped[(x, y)] = self.bits[((dim - x - 1), y)]
                }
            }
            flipped
        } else {
            self.bits.clone()
        };

        // rotate `ori.1` times
        for _i in 0..ori.1 {
            let mut rotated = Array2::<i8>::zeros(bits.dim());
            for x in 0..dim {
                for y in 0..dim {
                    rotated[(x, y)] = bits[(dim - 1 - y, x)]
                }
            }
            bits = rotated
        }

        Tile { bits }
    }

    fn top(&self) -> TileSide {
        TileSide(self.bits.slice(s![0, ..]).iter().copied().collect())
    }

    fn bottom(&self) -> TileSide {
        TileSide(
            self.bits
                .slice(s![self.bits.dim().0 - 1, ..])
                .iter()
                .copied()
                .collect(),
        )
    }

    fn left(&self) -> TileSide {
        TileSide(self.bits.slice(s![.., 0]).iter().copied().collect())
    }

    fn right(&self) -> TileSide {
        TileSide(
            self.bits
                .slice(s![.., self.bits.dim().1 - 1])
                .iter()
                .copied()
                .collect(),
        )
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

    fn solve(&self) -> Puzzle {
        let mut puzzle = Puzzle::new();
        let (tid, tile) = self.tiles.iter().next().unwrap();
        puzzle.insert((0, 0), (*tid, tile.clone()));
        let mut empty_pos = HashSet::new();
        empty_pos.insert((1, 0));
        empty_pos.insert((-1, 0));
        empty_pos.insert((0, 1));
        empty_pos.insert((0, -1));
        if !self.solve_helper(&mut puzzle, empty_pos) {
            panic!();
        }
        puzzle
    }

    fn solve_helper(
        &self,
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
            for (tid, tile) in &self.tiles {
                // If this tile is already in use, skip.
                if puzzle.values().any(|(used_tid, _)| used_tid == tid) {
                    continue;
                }

                // Try each orientation for this tile.
                for ori in &ORIENTATIONS {
                    let tile = tile.transformed(ori);

                    // If any neighbor doesn't match, skip.
                    if let Some((_, neighbor)) =
                        puzzle.get(&(empty_pos.0 + 1, empty_pos.1))
                    {
                        if neighbor.top() != tile.bottom() {
                            continue;
                        }
                    }
                    if let Some((_, neighbor)) =
                        puzzle.get(&(empty_pos.0 - 1, empty_pos.1))
                    {
                        if neighbor.bottom() != tile.top() {
                            continue;
                        }
                    }
                    if let Some((_, neighbor)) =
                        puzzle.get(&(empty_pos.0, empty_pos.1 + 1))
                    {
                        if neighbor.left() != tile.right() {
                            continue;
                        }
                    }
                    if let Some((_, neighbor)) =
                        puzzle.get(&(empty_pos.0, empty_pos.1 - 1))
                    {
                        if neighbor.right() != tile.left() {
                            continue;
                        }
                    }

                    // Insert into puzzle; add new empty positions
                    puzzle.insert(empty_pos, (*tid, tile));

                    let mut new_empty_pos = empty_positions.clone();
                    // Push all neighboring positions. Don't bother to check if they're already
                    // occupied - we check that when we use it.
                    new_empty_pos.insert((empty_pos.0 + 1, empty_pos.1));
                    new_empty_pos.insert((empty_pos.0 - 1, empty_pos.1));
                    new_empty_pos.insert((empty_pos.0, empty_pos.1 + 1));
                    new_empty_pos.insert((empty_pos.0, empty_pos.1 - 1));
                    if self.solve_helper(puzzle, new_empty_pos) {
                        return true;
                    }

                    // Rest of the puzzle didn't work out with this placement. Undo and try the next.
                    puzzle.remove(&empty_pos);
                }
            }
            // No tile fit this position; possibly because we're at an edge. Move to the next position.
        }

        // No more empty positions. We've finished the puzzle iff all tiles have been placed.
        puzzle.len() == self.tiles.len()
    }

    fn render(puzzle: &Puzzle) -> Tile {
        // Find edges
        let mut minx = i16::MAX;
        let mut miny = i16::MAX;
        let mut maxx = i16::MIN;
        let mut maxy = i16::MIN;
        for (y, x) in puzzle.keys() {
            minx = std::cmp::min(minx, *x);
            miny = std::cmp::min(miny, *y);
            maxx = std::cmp::max(maxx, *x);
            maxy = std::cmp::max(maxy, *y);
        }

        let (_, tile) = puzzle.values().next().unwrap();
        let tile_height = tile.bits.dim().0 as i16 - 2;
        let tile_width = tile.bits.dim().1 as i16 - 2;

        // Render
        let width = ((maxx - minx + 1) * tile_width) as usize;
        let height = ((maxy - miny + 1) * tile_height) as usize;
        let mut bits = Array2::<i8>::zeros((height, width));
        //assert_eq!(width, height);
        for y in 0..height {
            for x in 0..width {
                let tile_pos = (
                    y as i16 / tile_height + miny,
                    x as i16 / tile_width + minx,
                );
                let (_, tile) = puzzle.get(&tile_pos).unwrap();
                bits[(y, x)] = tile.bits[(
                    ((y as i16 % tile_height) + 1) as usize,
                    ((x as i16 % tile_width) + 1) as usize,
                )];
            }
        }
        Tile { bits }
    }
}

pub fn part1(input: &str) -> u64 {
    let ts = TileSet::new(input);
    let puzzle = ts.solve();

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

fn monsters_in(tile: &Tile) -> u64 {
    let monster = Tile::monster();
    ORIENTATIONS
        .iter()
        .map(|ori| {
            let tile = tile.transformed(ori);
            let mut count = 0;
            for x in 0..tile.bits.dim().1 {
                for y in 0..tile.bits.dim().0 {
                    if tile.contains(&monster, &(x as i16, y as i16)) {
                        count += 1
                    }
                }
            }
            count
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let ts = TileSet::new(input);
    let puzzle = ts.solve();
    let image = TileSet::render(&puzzle);

    let monster_count = monsters_in(&image);
    //println!("monster count: {}", monster_count);
    image.bits.iter().filter(|x| x == &&1).count() as u64
        - Tile::monster().bits.iter().filter(|x| x == &&1).count() as u64
            * monster_count
}

#[cfg(test)]
#[test]
fn test_sides() {
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
        ts.tiles.get(&2311).unwrap().top().0,
        vec![0, 0, 1, 1, 0, 1, 0, 0, 1, 0]
    );
    assert_eq!(
        ts.tiles.get(&2311).unwrap().left().0,
        vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0]
    );
    assert_eq!(
        ts.tiles.get(&2311).unwrap().bottom().0,
        vec![0, 0, 1, 1, 1, 0, 0, 1, 1, 1]
    );
    assert_eq!(
        ts.tiles.get(&2311).unwrap().right().0,
        vec![0, 0, 0, 1, 0, 1, 1, 0, 0, 1]
    );
}

#[cfg(test)]
#[test]
fn test_transforms() {
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
        ts.tiles
            .get(&2311)
            .unwrap()
            .transformed(&(false, 1))
            .top()
            .0,
        vec![0, 1, 0, 0, 1, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        ts.tiles
            .get(&2311)
            .unwrap()
            .transformed(&(false, 2))
            .top()
            .0,
        vec![1, 1, 1, 0, 0, 1, 1, 1, 0, 0]
    );
    assert_eq!(
        ts.tiles
            .get(&2311)
            .unwrap()
            .transformed(&(false, 3))
            .top()
            .0,
        vec![0, 0, 0, 1, 0, 1, 1, 0, 0, 1]
    );

    assert_eq!(
        ts.tiles.get(&2311).unwrap().transformed(&(true, 0)).top().0,
        vec![0, 0, 1, 1, 1, 0, 0, 1, 1, 1]
    );
}

#[cfg(test)]
#[test]
fn test_alignment() {
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
        ts.tiles.get(&1).unwrap().right(),
        ts.tiles.get(&2).unwrap().left()
    );
}

#[cfg(test)]
#[test]
fn test_part1() {
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

#[cfg(test)]
#[test]
fn test_contains() {
    let input = "\
..                 # 
.#    ##    ##    ###
. #  #  #  #  #  #   ";
    let tile = Tile::new(input);
    assert!(!tile.contains(&Tile::monster(), &(0, 0)));
    assert!(tile.contains(&Tile::monster(), &(1, 0)));
    assert!(!tile.contains(&Tile::monster(), &(0, 1)));
}

#[cfg(test)]
#[test]
fn test_monsters_in() {
    let input = "\
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###";
    let tile = Tile::new(input);
    assert_eq!(monsters_in(&tile), 2);
}

#[cfg(test)]
#[test]
fn test_render() {
    let mut puzzle = Puzzle::new();
    puzzle.insert(
        (0, 0),
        (
            1,
            Tile::new(
                "\
#.#.#
..#..
#.#.#
.....
#.#.#",
            ),
        ),
    );
    assert_eq!(
        TileSet::render(&puzzle).bits,
        Tile::new(
            "\
.#.
.#.
..."
        )
        .bits
    );

    puzzle.insert(
        (1, 0),
        (
            2,
            Tile::new(
                "\
#.#.#
.....
#####
.....
#.#.#",
            ),
        ),
    );
    assert_eq!(
        TileSet::render(&puzzle).bits,
        Tile::new(
            "\
.#.
.#.
...
...
###
..."
        )
        .bits
    );

    puzzle.insert(
        (0, 1),
        (
            3,
            Tile::new(
                "\
#.#.#
..#..
#####
..#..
#.#.#",
            ),
        ),
    );
    puzzle.insert(
        (1, 1),
        (
            4,
            Tile::new(
                "\
#.#.#
.###.
#####
..#..
#.#.#",
            ),
        ),
    );
    assert_eq!(
        TileSet::render(&puzzle).bits,
        Tile::new(
            "\
.#..#.
.#.###
....#.
...###
######
....#."
        )
        .bits
    );
}

#[cfg(test)]
#[test]
fn test_part2() {
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
    let ts = TileSet::new(input);
    let puzzle = ts.solve();
    let image = TileSet::render(&puzzle);
    let expected = Tile::new(
        "\
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
",
    );
    for ori in &ORIENTATIONS {
        println!("{:?}\n", image.transformed(ori).bits);
    }
    assert!(ORIENTATIONS
        .iter()
        .any(|ori| image.transformed(ori).bits == expected.bits));

    assert_eq!(part2(input), 273);
}
