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

fn number_not_following_rule(nums: &[u64], n: usize) -> u64 {
    let idx = (n..nums.len())
        .find(|i| !has_sum_operands(&nums, *i, n))
        .unwrap();
    nums[idx]
}

pub fn part1(input: &str, n: usize) -> u64 {
    let nums: Vec<_> =
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    number_not_following_rule(&nums, n)
}

fn sum_of_bounds(nums: &[u64], target: u64) -> u64 {
    for low in 0..nums.len() {
        let mut sum = nums[low];
        for high in (low + 1)..nums.len() {
            sum += nums[high];
            if sum == target {
                return nums[low..=high].iter().min().unwrap()
                    + nums[low..=high].iter().max().unwrap();
            }
            if sum > target {
                break;
            }
        }
    }
    panic!("Not found");
}

pub fn part2(input: &str, n: usize) -> usize {
    let nums: Vec<_> =
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let target = number_not_following_rule(&nums, n);
    sum_of_bounds(&nums, target) as usize
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
        assert_eq!(part2(input, 5), 62);
    }
}
