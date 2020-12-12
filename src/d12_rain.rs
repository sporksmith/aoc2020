use std::ops::{Add, Mul};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Turn(Turn, u16),
    Bearing(Bearing, u16),
    Forward(u16),
}

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

impl Bearing {
    fn to_vector(self) -> Vector {
        match self {
            Bearing::N => Vector { x: 0, y: 1 },
            Bearing::E => Vector { x: 1, y: 0 },
            Bearing::S => Vector { x: 0, y: -1 },
            Bearing::W => Vector { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector {
    fn rotate(self, t: Turn, amt: i32) -> Vector {
        let amt = (match t {
            Turn::L => amt,
            Turn::R => -amt,
        } as f64)
            .to_radians();
        let sin = amt.sin();
        let cos = amt.cos();
        let x = self.x as f64;
        let y = self.y as f64;
        Vector {
            x: (cos * x - sin * y) as i32,
            y: (sin * x + cos * y) as i32,
        }
    }
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
    pos: Vector,
    bearing: Vector,
}

impl BoatState {
    pub fn go_part1(&mut self, d: &Direction) {
        match d {
            Direction::Turn(t, a) => {
                self.bearing = self.bearing.rotate(*t, *a as i32)
            }
            Direction::Bearing(b, a) => {
                self.pos = self.pos + b.to_vector() * (*a as i32)
            }
            Direction::Forward(a) => {
                self.pos = self.pos + self.bearing * (*a as i32)
            }
        }
    }
}

pub fn part1(directions: &[Direction]) -> u64 {
    let mut boat = BoatState {
        pos: Vector { x: 0, y: 0 },
        bearing: Bearing::E.to_vector(),
    };
    for d in directions {
        boat.go_part1(d);
    }
    (boat.pos.x.abs() + boat.pos.y.abs()) as u64
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
            pos: Vector { x: 0, y: 0 },
            bearing: Bearing::E.to_vector(),
        };
        boat.go_part1(&input[0]);
        assert_eq!(
            boat,
            BoatState {
                pos: Vector { x: 10, y: 0 },
                bearing: Bearing::E.to_vector(),
            }
        );
        boat.go_part1(&input[1]);
        assert_eq!(
            boat,
            BoatState {
                pos: Vector { x: 10, y: 3 },
                bearing: Bearing::E.to_vector(),
            }
        );
        boat.go_part1(&input[2]);
        assert_eq!(
            boat,
            BoatState {
                pos: Vector { x: 17, y: 3 },
                bearing: Bearing::E.to_vector(),
            }
        );
        boat.go_part1(&input[3]);
        assert_eq!(
            boat,
            BoatState {
                pos: Vector { x: 17, y: 3 },
                bearing: Bearing::S.to_vector(),
            }
        );
        boat.go_part1(&input[4]);
        assert_eq!(
            boat,
            BoatState {
                pos: Vector { x: 17, y: -8 },
                bearing: Bearing::S.to_vector(),
            }
        );

        assert_eq!(part1(&input), 25);
    }
}
