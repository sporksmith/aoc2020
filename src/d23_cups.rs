// Prevent Clippy from complaining about &Circle arguments
#![allow(clippy::ptr_arg)]

type Cup = i8;
type Circle = Vec<Cup>;

fn parse_circle(input: &str) -> Circle {
    input.trim().bytes().map(|b| (b - b'0') as Cup).collect()
}

fn one_move(circle: &Circle) -> Circle {
    let current = circle[0];

    let mut dest = current - 1;
    while !circle[4..].contains(&dest) {
        dest -= 1;
        if dest <= 0 {
            dest = 9;
        }
    }

    let mut rv = Vec::with_capacity(circle.len());
    for cup in &circle[4..] {
        rv.push(*cup);
        if *cup == dest {
            circle[1..4].iter().for_each(|c| rv.push(*c));
        }
    }
    rv.push(current);
    rv
}

fn canonicalize(circle: &Circle) -> String {
    let mut s = String::with_capacity(circle.len() - 1);
    let conv = |c: Cup| (b'0' + c as u8) as char;

    // Everything after 1
    circle
        .iter()
        .skip_while(|c| **c != 1)
        .skip(1)
        .for_each(|c| s.push(conv(*c)));

    // Everything before 1
    circle
        .iter()
        .take_while(|c| **c != 1)
        .for_each(|c| s.push(conv(*c)));
    s
}

pub fn part1(input: &str) -> String {
    let mut circle = parse_circle(input);
    for _ in 0..100 {
        circle = one_move(&circle);
    }
    canonicalize(&circle)
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_example() {
        let input = "389125467";
        let circle = parse_circle(input);
        assert_eq!(
            circle,
            [3, 8, 9, 1, 2, 5, 4, 6, 7]
                .iter()
                .copied()
                .collect::<Circle>()
        );
        assert_eq!(one_move(&circle), parse_circle("289154673"));

        assert_eq!(canonicalize(&parse_circle("583741926")), "92658374");
        assert_eq!(part1(input), "67384529");
    }
}
