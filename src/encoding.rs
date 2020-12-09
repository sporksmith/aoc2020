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

pub fn part1(input: &str, n: usize) -> usize {
    let nums: Vec<_> =
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let idx = (n..nums.len())
        .find(|i| !has_sum_operands(&nums, *i, n))
        .unwrap();
    nums[idx] as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
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
576";
        assert_eq!(part1(input, 5), 127);
    }
}
