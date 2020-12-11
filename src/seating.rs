#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Grid {
    positions: Vec<Position>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn idx(&self, col: isize, row: isize) -> Option<usize> {
        if !(0..self.rows as isize).contains(&row)
            || !(0..self.cols as isize).contains(&col)
        {
            return None;
        }
        Some((row as usize) * self.cols + col as usize)
    }

    // Note - returns "Floor" for out of bounds
    pub fn get(&self, col: isize, row: isize) -> Position {
        match self.idx(col, row) {
            Some(i) => self.positions[i],
            None => Position::Floor,
        }
    }

    pub fn set(&mut self, col: isize, row: isize, pos: Position) {
        let idx = self.idx(col, row).unwrap();
        self.positions[idx] = pos
    }

    // Optionally takes storage for `next` (which must match the shape of `self`), allowing us to
    // avoid hitting the allocator. Returns the next state.
    pub fn step(&self, next: Option<Grid>) -> Grid {
        let mut next = match next {
            Some(next) => next,
            None => self.clone(),
        };
        debug_assert_eq!(self.cols, next.cols);
        debug_assert_eq!(self.rows, next.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let row = row as isize;
                let col = col as isize;

                let pos = self.get(col, row);
                if pos == Position::Floor {
                    next.set(col, row, pos);
                    continue;
                }

                #[rustfmt::skip]
                let neighbor_diffs = [
                    (-1,1), (0,1), (1,1),
                    (-1,0),        (1,0),
                    (-1,-1),(0,-1),(1,-1)];
                let occupied_neighbor_count = neighbor_diffs
                    .iter()
                    .map(|(dcol, drow)| self.get(col + dcol, row + drow))
                    .filter(|p| p == &Position::Occupied)
                    .count();
                let next_pos = if pos == Position::Occupied
                    && occupied_neighbor_count >= 4
                {
                    Position::Empty
                } else if pos == Position::Empty && occupied_neighbor_count == 0
                {
                    Position::Occupied
                } else {
                    pos
                };
                next.set(col, row, next_pos);
            }
        }
        next
    }

    pub fn occupied(&self) -> usize {
        let mut count = 0;
        for col in 0..self.cols {
            for row in 0..self.rows {
                if self.get(col as isize, row as isize) == Position::Occupied {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn parse(input: &str) -> Grid {
    let cols = input.find('\n').unwrap();
    // XXX: Fudge the numerator to handle a missing last endline.  Breaks for a sing-columng grid.
    let rows = (input.len() + 1) / (cols + 1);
    let size = rows * cols;

    let mut positions = Vec::with_capacity(rows * cols);
    for line in input.lines() {
        debug_assert_eq!(line.len(), cols);
        for pos in line.chars().map(|c| match c {
            '.' => Position::Floor,
            '#' => Position::Occupied,
            'L' => Position::Empty,
            _ => panic!("Unexpected char"),
        }) {
            positions.push(pos);
        }
    }
    debug_assert_eq!(positions.len(), size);
    Grid {
        rows,
        cols,
        positions,
    }
}

pub fn part1(zero: &Grid) -> usize {
    let mut prev = zero.clone();
    let mut curr = zero.step(None);
    while curr != prev {
        let next = curr.step(Some(prev));
        prev = curr;
        curr = next;
    }
    curr.occupied()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let zero = parse(
            "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );
        let one = parse(
            "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );
        let two = parse(
            "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        );
        assert_eq!(zero.step(None), one);
        assert_eq!(one.step(None), two);
        assert_eq!(part1(&zero), 37);
    }
}
