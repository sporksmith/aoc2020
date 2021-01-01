fn transform(subject: u64, value: u64) -> u64 {
    (value * subject) % 20201227
}

fn transformn(subject: u64, n: u64) -> u64 {
    let mut value = 1;
    for _ in 0..n {
        value = transform(subject, value)
    }
    value
}

fn find_loop_count(pk: u64) -> u64 {
    let mut value = 1;
    let mut loops = 0;
    while value != pk {
        value = transform(7, value);
        loops += 1;
    }
    loops
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let pk1: u64 = lines.next().unwrap().parse().unwrap();
    let pk2: u64 = lines.next().unwrap().parse().unwrap();
    let sk1 = find_loop_count(pk1);
    transformn(pk2, sk1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_count() {
        assert_eq!(find_loop_count(5764801), 8);
        assert_eq!(find_loop_count(17807724), 11);
        assert_eq!(transformn(17807724, 8), 14897079);
        assert_eq!(transformn(5764801, 11), 14897079);
        assert_eq!(part1("17807724\n5764801"), 14897079);
    }
}
