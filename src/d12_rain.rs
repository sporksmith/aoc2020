#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Turn {
    L,
    R,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Bearing {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Turn(Turn, u16),
    Bearing(Bearing, u16),
    Forward(u16),
}

pub fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let (kind, amount) = line.split_at(1);
            let amount = amount.parse::<u16>().unwrap();
            match kind.chars().next().unwrap() {
                'L' => Direction::Turn(Turn::L, amount),
                'R' => Direction::Turn(Turn::R, amount),
                'E' => Direction::Bearing(Bearing::E, amount),
                'S' => Direction::Bearing(Bearing::S, amount),
                'W' => Direction::Bearing(Bearing::W, amount),
                'N' => Direction::Bearing(Bearing::N, amount),
                'F' => Direction::Forward(amount),
                _ => panic!("Bad direction"),
            }
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoatState {
    x: i32,
    y: i32,
    b: Bearing,
}

impl BoatState {
    fn go_bearing(&mut self, b: Bearing, a: u32) {
        let a = a as i32;
        match b {
            Bearing::N => self.y += a,
            Bearing::S => self.y -= a,
            Bearing::E => self.x += a,
            Bearing::W => self.x -= a,
        };
    }

    pub fn go(&mut self, d: &Direction) {
        match d {
            Direction::Turn(t, a) => self.b = self.b.turn(*t, *a),
            Direction::Bearing(b, a) => self.go_bearing(*b, *a as u32),
            Direction::Forward(a) => self.go_bearing(self.b, *a as u32),
        }
    }

    pub fn after(&self, d: &Direction) -> BoatState {
        let mut res = self.clone();
        res.go(d);
        res
    }
}

impl Bearing {
    /// ```
    /// use aoc2020::d12_rain::*;
    /// assert_eq!(Bearing::N.turn(Turn::L, 90), Bearing::W);
    /// assert_eq!(Bearing::E.turn(Turn::L, 180), Bearing::W);
    ///
    /// assert_eq!(Bearing::N.turn(Turn::R, 90), Bearing::E);
    /// assert_eq!(Bearing::E.turn(Turn::R, 270), Bearing::N);
    /// ```
    pub fn turn(&self, d: Turn, a: u16) -> Bearing {
        debug_assert_eq!(a % 90, 0);
        let dirs = match d {
            Turn::L => &[Bearing::N, Bearing::W, Bearing::S, Bearing::E],
            Turn::R => &[Bearing::N, Bearing::E, Bearing::S, Bearing::W],
        };
        // There *must* be a more succinct way to express this!
        let (offset, _) =
            dirs.iter().enumerate().find(|(_, x)| x == &self).unwrap();
        dirs[(offset + (a as usize) / 90) % dirs.len()]
    }
}

pub fn part1(directions: &[Direction]) -> u64 {
    let mut boat = BoatState {
        x: 0,
        y: 0,
        b: Bearing::E,
    };
    for d in directions {
        boat.go(d);
    }
    (boat.x.abs() + boat.y.abs()) as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = parse(
            "\
F10
N3
F7
R90
F11",
        );
        let mut boat = BoatState {
            x: 0,
            y: 0,
            b: Bearing::E,
        };
        boat.go(&input[0]);
        assert_eq!(
            boat,
            BoatState {
                x: 10,
                y: 0,
                b: Bearing::E
            }
        );
        boat.go(&input[1]);
        assert_eq!(
            boat,
            BoatState {
                x: 10,
                y: 3,
                b: Bearing::E
            }
        );
        boat.go(&input[2]);
        assert_eq!(
            boat,
            BoatState {
                x: 17,
                y: 3,
                b: Bearing::E
            }
        );
        boat.go(&input[3]);
        assert_eq!(
            boat,
            BoatState {
                x: 17,
                y: 3,
                b: Bearing::S
            }
        );
        boat.go(&input[4]);
        assert_eq!(
            boat,
            BoatState {
                x: 17,
                y: -8,
                b: Bearing::S
            }
        );

        assert_eq!(part1(&input), 25);
    }
}
