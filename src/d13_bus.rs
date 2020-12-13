use num::Integer;

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let arrival: u32 = lines.next().unwrap().parse().unwrap();
    let (wait, n): (u32, u32) = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|n| n.parse().unwrap())
        .map(|n| (n - (arrival % n), n))
        .min()
        .unwrap();
    (wait * n) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next(); // ignore
    let constraints: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| match x {
            "x" => None,
            _ => Some(x.parse::<u64>().unwrap()),
        })
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| (i as u64, x.unwrap()))
        .collect();
    let mut delta: u64 = 1;
    let mut t: u64 = 0;
    for (offset, dt) in &constraints {
        // Move time forward by delta until we find a time that has the right offset for this
        // train.
        loop {
            if (t + *offset) % dt == 0 {
                // This time produces the right offset for this train.
                break;
            }
            t += delta;
        }
        // From now on take steps that will preserve the offset for this train, while still
        // preserving the offset of all previously processed trains. i.e. this should be a multiple
        // of this train's dt and all previous train's dt's. Needs to be the least such multiple so
        // that we don't skip potentially valid answers.
        delta = delta.lcm(dt);
    }
    t
}

// First attempt, for posterity. I estimate should get the right answer for the puzzle input in
// ~7h.
pub fn part2_naive(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next(); // ignore
    let mut constraints: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| match x {
            "x" => None,
            _ => Some(x.parse::<u64>().unwrap()),
        })
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| (i as u64, x.unwrap()))
        .collect();
    // Sort by time delta, largest first.
    constraints.sort_by(|(_, dt_lhs), (_, dt_rhs)| dt_rhs.cmp(dt_lhs));
    let delta: u64 = constraints[0].1;
    let mut t: u64 = constraints[0].1 - (constraints[0].0 % constraints[0].1);
    let mut i = 0u64;
    loop {
        if constraints[1..constraints.len()]
            .iter()
            .all(|(offset, dt)| (t + offset) % dt == 0)
        {
            break;
        }
        t += delta;
        i += 1;
        if i % 1000000000u64 == 0 {
            println!("{}", t);
        }
    }
    t
}

#[cfg(test)]
#[test]
fn test_example() {
    assert_eq!(part1("939\n7,13,x,x,59,x,31,19"), 295);
    assert_eq!(part2("939\n7,13,x,x,59,x,31,19"), 1068781);
    assert_eq!(part2_naive("939\n7,13,x,x,59,x,31,19"), 1068781);

    // non-coprime inputs
    // ht @ https://www.reddit.com/r/adventofcode/comments/kc94h1/2020_day_13_part_2_generalization/
    assert_eq!(
        part2("939\n14,x,x,x,335,x,x,x,39,x,x,x,x,x,x,x,x,187,19"),
        124016326
    );
    assert_eq!(
        part2("939\n73,x,x,x,x,x,x,67,x,25,x,x,x,x,x,343,x,x,9"),
        369373941
    );
    assert_eq!(part2("\n1997,x,x,x,x,x,x,1747,x,x,x,x,x,2003,x,x,x,x,x,x,1883,x,x,x,x,x,1667,x,x,x,x,x,x,x,1701"), 4756544012204563475);
}
