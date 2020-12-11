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
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

fn part2_helper(prev: u64, nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return 1;
    }
    if nums[0] - prev > 3 {
        return 0;
    }
    let ways_with_next = part2_helper(nums[0], &nums[1..nums.len()]);
    let ways_without_next = if nums[1] - prev > 3 {
        0
    } else {
        part2_helper(prev, &nums[1..nums.len()])
    };
    ways_with_next + ways_without_next
}

pub fn part2(nums: &[u64]) -> u64 {
    part2_helper(nums[0], &nums[1..nums.len()])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[0, 3]), 1);
        assert_eq!(part2(&[0, 3, 6]), 1);
        assert_eq!(part2(&[0, 1, 3, 6]), 2);
        assert_eq!(part2(&[0, 1, 2, 3, 6]), 4);
    }

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
        assert_eq!(part2(&input), 8);

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
        assert_eq!(part2(&input), 19208);
    }
}
