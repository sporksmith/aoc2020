// Output is *sorted*
pub fn parse(input: &str) -> Vec<u64> {
    let mut res: Vec<_> =
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    // Starting voltage is 0.
    res.push(0);
    res.sort_unstable();
    // Final voltage is 3 greater than last adapter
    res.push(res.last().unwrap() + 3);
    res
}

// Assumes `nums` is sorted
pub fn part1(nums: &[u64]) -> u64 {
    let (mut ones, mut threes) = (0, 0);
    for diff in nums.windows(2).map(|x| x[1] - x[0]) {
        println!("{}", diff);
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = parse(
            "\
16
10
15
5
1
11
7
19
6
12
4",
        );
        assert_eq!(part1(&input), 35);

        let input = parse(
            "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        );
        assert_eq!(part1(&input), 220);
    }
}
