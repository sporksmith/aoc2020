use lazy_static::lazy_static;
use std::collections::HashSet;

type Pos = (i32, i32, i32, i32);
type State = HashSet<Pos>;

lazy_static! {
    static ref DELTAS3: Vec<Pos> = {
        let mut d = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    d.push((dx, dy, dz, 0));
                }
            }
        }
        d
    };
    static ref DELTAS4: Vec<Pos> = {
        let mut d = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        d.push((dx, dy, dz, dw));
                    }
                }
            }
        }
        d
    };
}

/*
fn state2string(state: &State) -> String {
    use std::cmp::{min,max};
    let mut minx=i32::MAX;
    let mut maxx=i32::MIN;
    let mut miny=i32::MAX;
    let mut maxy=i32::MIN;
    let mut minz=i32::MAX;
    let mut maxz=i32::MIN;
    for (x,y,z) in state {
        minx = min(minx,*x);
        miny = min(miny,*y);
        minz = min(minz,*z);
        maxx = max(maxx,*x);
        maxy = max(maxy,*y);
        maxz = max(maxz,*z);
    }
    let mut res = String::new();
    for z in minz..=maxz {
        res.push_str(&format!("z={}\n", z));
        for y in miny..=maxy {
            for x in minx..=maxx {
                res.push( if state.contains(&(x,y,z)) {
                    '#'
                } else {
                    '.'
                });
            }
            res.push('\n');
        }
    }
    res
}
*/

fn candidates(state: &State, deltas: &[Pos]) -> HashSet<Pos> {
    let mut res = HashSet::<Pos>::new();
    for pos in state {
        res.insert(*pos);
        for (dx, dy, dz, dw) in deltas.iter() {
            res.insert((pos.0 + dx, pos.1 + dy, pos.2 + dz, pos.3 + dw));
        }
    }
    res
}

fn next(state: &State, deltas: &[Pos]) -> State {
    let mut next_state = State::new();
    for (x, y, z, w) in candidates(state, deltas) {
        let neighbor_count: u8 = deltas
            .iter()
            .map(|(dx, dy, dz, dw)| {
                if state.contains(&(x + dx, y + dy, z + dz, w + dw)) {
                    1
                } else {
                    0
                }
            })
            .sum();
        let active = state.contains(&(x, y, z, w));
        if neighbor_count == 3 || active && neighbor_count == 2 {
            let pos = (x, y, z, w);
            // println!("  Inserting {:?}", pos);
            next_state.insert(pos);
        }
    }
    next_state
}

fn parse(input: &str) -> State {
    let mut res = State::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.insert((x as i32, y as i32, 0, 0));
            }
        }
    }
    res
}

pub fn part1(input: &str) -> usize {
    let mut state = parse(input);
    //println!("Initial:\n{}", state2string(&state));
    for _i in 0..6 {
        //println!("Round {}", i);
        state = next(&state, &DELTAS3);
        //println!("{}", state2string(&state));
    }
    state.len()
}

pub fn part2(input: &str) -> usize {
    let mut state = parse(input);
    //println!("Initial:\n{}", state2string(&state));
    for _i in 0..6 {
        //println!("Round {}", i);
        state = next(&state, &DELTAS4);
        //println!("{}", state2string(&state));
    }
    state.len()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "\
.#.
..#
###";
    assert_eq!(part1(input), 112);
    assert_eq!(part2(input), 848);
}
