// Prevent Clippy from complaining about &Circle arguments
#![allow(clippy::ptr_arg)]

type Cup = i32;
type Circle = Vec<Cup>;

fn parse_circle(input: &str) -> Circle {
    input.trim().bytes().map(|b| (b - b'0') as Cup).collect()
}

fn one_move(circle: &Circle) -> Circle {
    let current = circle[0];

    let mut dest = current - 1;
    while dest <= 0 || circle[..4].contains(&dest) {
        dest -= 1;
        if dest <= 0 {
            dest = circle[4..].iter().copied().max().unwrap();
            break;
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

fn expand(circle: &mut Circle) {
    let highest = circle.iter().copied().max().unwrap();
    circle.reserve(1_000_000);
    for i in highest..=1_000_000 {
        circle.push(i);
    }
}

fn product_after_cup1(circle: &Circle) -> u64 {
    let mut it = circle.iter().cycle().skip_while(|c| **c != 1).skip(1);
    let x = *it.next().unwrap();
    let y = *it.next().unwrap();
    x as u64 * y as u64
}

pub fn part2_slow(input: &str) -> u64 {
    let mut circle = parse_circle(input);
    expand(&mut circle);
    for i in 0..10_000_000 {
        circle = one_move(&circle);
        if i % 1000 == 0 {
            println!("Turn {}", i);
        }
    }
    product_after_cup1(&circle)
}

#[derive(Debug)]
struct BigCircle {
    current: Cup,
    max: Cup,
    // This is essentially a highly specialized linked hash map.
    // The cup after c is `next_cup[c]`.
    next_cup: Vec<Cup>,
}

impl BigCircle {
    fn insert_after(&mut self, prev: Cup, next: Cup) {
        let next_next = self.next_cup[prev as usize];
        self.next_cup[prev as usize] = next;
        self.next_cup[next as usize] = next_next;
    }

    fn pop_after(&mut self, prev: Cup) -> Cup {
        let next = self.next_cup[prev as usize];
        self.next_cup[prev as usize] = self.next_cup[next as usize];
        next
    }

    fn new(circle: Circle) -> BigCircle {
        let max = circle.iter().copied().max().unwrap();

        // Create with dummy data.
        let mut next_cup = Vec::new();
        // We need one more than `max` so that we can index directly by cup #.
        next_cup.resize((max + 1) as usize, 0);

        // Fill from circle
        let mut prev_cup = circle[0];
        for cup in &circle[1..] {
            next_cup[prev_cup as usize] = *cup;
            prev_cup = *cup;
        }

        // Close circle
        next_cup[prev_cup as usize] = circle[0];

        BigCircle {
            current: circle[0],
            max,
            next_cup,
        }
    }

    fn extend(&mut self, max: Cup) {
        assert!(max > self.max);

        let old_max = self.max;
        self.max = max;

        // We need one more than `max` so that we can index directly by cup #.
        self.next_cup.resize((max + 1) as usize, 0);

        // Initialize the prev cup to the one just before 'current'
        let mut prev_cup = self.current;
        while self.next_cup[prev_cup as usize] != self.current {
            prev_cup = self.next_cup[prev_cup as usize];
        }

        // Fill to `max`
        for cup in (old_max + 1)..=max {
            self.next_cup[prev_cup as usize] = cup;
            prev_cup = cup;
        }

        // Re-close the circle
        self.next_cup[prev_cup as usize] = self.current;
    }

    fn p1_summarize(&self) -> String {
        let mut s = String::with_capacity(self.max as usize);
        let mut prev = 1;
        loop {
            let next = self.next_cup[prev as usize];
            if next == 1 {
                break;
            }
            assert!(next < 10);
            s.push((b'0' + next as u8) as char);
            prev = next;
        }
        s
    }

    fn step(&mut self) {
        let a = self.pop_after(self.current);
        let b = self.pop_after(self.current);
        let c = self.pop_after(self.current);

        let mut dest = self.current;
        while [self.current, a, b, c].contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = self.max;
            }
        }
        self.insert_after(dest, c);
        self.insert_after(dest, b);
        self.insert_after(dest, a);

        self.current = self.next_cup[self.current as usize]
    }
}

pub fn part2(input: &str) -> u64 {
    let mut big_circle = BigCircle::new(parse_circle(input));
    big_circle.extend(1_000_000);
    for _ in 0..10_000_000 {
        big_circle.step();
    }
    let a = big_circle.pop_after(1);
    let b = big_circle.pop_after(1);
    a as u64 * b as u64
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
        //assert_eq!(part2(input), 149245887792);
    }

    #[test]
    fn test_big_circle() {
        let mut big_circle = BigCircle::new(parse_circle("123"));
        assert_eq!(big_circle.pop_after(1), 2);
        assert_eq!(big_circle.p1_summarize(), "3");

        let mut big_circle = BigCircle::new(parse_circle("123"));
        assert_eq!(big_circle.pop_after(2), 3);
        assert_eq!(big_circle.p1_summarize(), "2");

        let mut big_circle = BigCircle::new(parse_circle("123"));
        assert_eq!(big_circle.pop_after(1), 2);
        assert_eq!(big_circle.pop_after(1), 3);
        big_circle.insert_after(1, 2);
        assert_eq!(big_circle.p1_summarize(), "2");
        big_circle.insert_after(1, 3);
        assert_eq!(big_circle.p1_summarize(), "32");

        let input = "389125467";
        let mut big_circle = BigCircle::new(parse_circle(input));
        big_circle.step();
        assert_eq!(big_circle.p1_summarize(), "54673289");
        big_circle.step();
        assert_eq!(big_circle.p1_summarize(), "32546789");

        let mut big_circle = BigCircle::new(parse_circle(input));
        for _ in 0..100 {
            big_circle.step();
        }
        assert_eq!(big_circle.p1_summarize(), "67384529");

        let mut big_circle = BigCircle::new(parse_circle("123"));
        big_circle.extend(6);
        println!("after extend {:?}", big_circle);
        assert_eq!(big_circle.p1_summarize(), "23456");

        // Slow
        // assert_eq!(part2(input), 149245887792);
    }
}
