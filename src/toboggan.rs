use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Tree,
}

#[derive(Debug)]
pub struct Map {
    cells: Vec<Cell>,
    width: usize,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;
    /// ```
    /// use aoc2020::toboggan as t;
    /// let map : t::Map = "..#\n\
    ///                     .#.\n".parse().unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim_end().split('\n').peekable();

        let width = lines.peek().ok_or_else(|| "Empty map".to_string())?.len();

        let mut cells = Vec::<Cell>::new();
        for line in lines {
            if line.len() != width {
                return Err(format!(
                    "Expected width {} got {}",
                    width,
                    line.len()
                )
                .into());
            }
            for c in line.chars() {
                cells.push(match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Tree,
                    _ => return Err("Bad cell".into()),
                });
            }
        }
        Ok(Map { width, cells })
    }
}

impl Map {
    /// ```
    /// use aoc2020::toboggan as t;
    /// let m : t::Map = "..#\n\
    ///                   .#.".parse().unwrap();
    /// // Top left corner
    /// assert_eq!(m.get(0, 0).unwrap(), &t::Cell::Empty);
    /// // Top right corner
    /// assert_eq!(m.get(0, 2).unwrap(), &t::Cell::Tree);
    /// // Bottom left corner
    /// assert_eq!(m.get(1, 0).unwrap(), &t::Cell::Empty);
    /// // Bottom right corner
    /// assert_eq!(m.get(1, 2).unwrap(), &t::Cell::Empty);
    /// // Repeated tree in 2nd row
    /// assert_eq!(m.get(1, 4).unwrap(), &t::Cell::Tree);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        // Map repeats infinitely every `width`
        let col = col % self.width;
        self.cells.get(row * self.width + col)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Angle {
    pub right: usize,
    pub down: usize,
}

/// ```
/// use aoc2020::toboggan as t;
/// let m : t::Map =
///     "..##.......\n\
///      #...#...#..\n\
///      .#....#..#.\n\
///      ..#.#...#.#\n\
///      .#...##..#.\n\
///      ..#.##.....\n\
///      .#.#.#....#\n\
///      .#........#\n\
///      #.##...#...\n\
///      #...##....#\n\
///      .#..#...#.#".parse().unwrap();
/// assert_eq!(t::trees_for_angle(&m, t::Angle{right: 3, down: 1}), 7);
/// ```
pub fn trees_for_angle(map: &Map, angle: Angle) -> usize {
    let mut count = 0;
    let mut row = 0;
    let mut col = 0;
    loop {
        col += angle.right;
        row += angle.down;
        if row >= map.height() {
            break;
        }
        if map.get(row, col).unwrap() == &Cell::Tree {
            count += 1;
        }
    }
    count
}
