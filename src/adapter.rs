use std::collections::HashMap;

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

pub fn part2(nums: &[u64]) -> u64 {
    part2_helper(&mut HashMap::new(), nums[0], 1, nums)
}

// Recursive helper. Memoized via memo_table to make not insanely expensive.  Returns the number of
// ways to connect the adapter `prev_val` to the adapters in `&nums[start_idx..]`. We pass
// `start_idx` explicitly instead of the subslice itself, so that we can use `(prev_val,
// start_idx)` as a cheap key into the memo table.
fn part2_helper(
    memo_table: &mut HashMap<(u64, usize), u64>,
    prev_val: u64,
    start_idx: usize,
    nums: &[u64],
) -> u64 {
    if let Some(res) = memo_table.get(&(prev_val, start_idx)) {
        // We've already computed this. Return the previous result.
        return *res;
    }

    // Alias for the slice of interest, for convenience.
    let slice = &nums[start_idx..nums.len()];

    // We should never be called with a zero-length slice (assuming sane original input).
    debug_assert_ne!(slice.len(), 0);

    // If the gap between the previous value and the first value in the slice is too large, then
    // there are no ways to arrange the slice.
    if slice[0] - prev_val > 3 {
        return 0;
    }
    // If there's only one value in the slice, then there's only one possibility.
    if slice.len() == 1 {
        return 1;
    }
    // Number of ways to arrange the slice if we include the first value of the slice.
    let ways_with_next =
        part2_helper(memo_table, slice[0], start_idx + 1, nums);
    // Number of ways to arrange the slice if we *don't* include the first value of the slice.
    let ways_without_next = if slice[1] - prev_val > 3 {
        0
    } else {
        part2_helper(memo_table, prev_val, start_idx + 1, nums)
    };
    let res = ways_with_next + ways_without_next;
    // Memoize recursive results, so that we don't need to recompute them later.
    memo_table.insert((prev_val, start_idx), res);
    res
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
