fn has_sum_operands(nums: &[u64], i: usize, n: usize) -> bool {
    for j in 1..=n {
        for k in (j + 1)..=n {
            if nums[i - j] + nums[i - k] == nums[i] {
                return true;
            }
        }
    }
    false
}

pub fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn part1(nums: &[u64], n: usize) -> u64 {
    let idx = (n..nums.len())
        .find(|i| !has_sum_operands(&nums, *i, n))
        .unwrap();
    nums[idx]
}

pub fn part2(nums: &[u64], n: usize) -> usize {
    let target = part1(&nums, n);
    let mut lo = 0;
    let mut hi = 1;
    let mut sum = nums[lo] + nums[hi];
    // While I had the general idea before, structure of this loop was definitely inspired by
    // seeing https://github.com/AxlLind/AdventOfCode2020/blob/537508ca5abc08198ed65cb8240ac9f174d37b7a/src/bin/09.rs#L18
    while sum != target {
        // Consider bumping `lo` up, being careful not to pass `hi`.
        if sum > target && ((lo + 1) <= hi) {
            sum -= nums[lo];
            lo += 1;
        } else {
            hi += 1;
            sum += nums[hi];
        }
    }
    let seq = &nums[lo..=hi];
    (seq.iter().max().unwrap() + seq.iter().min().unwrap()) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse(
            "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        );

        assert_eq!(part1(&input, 5), 127);
        assert_eq!(part2(&input, 5), 62);
    }
}
