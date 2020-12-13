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
        .map(|(i, x)| (i, x.unwrap()))
        .collect();
    let mut delta: u64 = 1;
    let mut t: u64 = 0;
    for (offset, dt) in &constraints {
        // Move time forward by delta until we find a time that has the right offset for this
        // train.
        loop {
            if (t + (*offset as u64)) % dt == 0 {
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

#[cfg(test)]
#[test]
fn test_example() {
    assert_eq!(part1("939\n7,13,x,x,59,x,31,19"), 295);
    assert_eq!(part2("939\n7,13,x,x,59,x,31,19"), 1068781);
}
