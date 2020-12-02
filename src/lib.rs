/// Finds two numbers from `xs` that sum to `target`.
///
/// ```
/// use aoc2020::find_sum_factors;
/// assert_eq!(find_sum_factors(&[1721, 979, 366, 299, 675, 1456],
///                             2020),
///            Some((1721, 299)));
/// assert_eq!(find_sum_factors(&[],
///                             2020),
///            None);
/// assert_eq!(find_sum_factors(&[1],
///                             2020),
///            None);
/// ```
pub fn find_sum_factors(xs: &[u32], target: u32) -> Option<(u32, u32)> {
    for i in xs {
        for j in xs.iter().skip(1) {
            if i+j == target {
                return Some((*i, *j))
            }
        }
    }
    None
}
