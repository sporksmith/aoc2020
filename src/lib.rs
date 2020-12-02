/// Finds `n` elements of `xs` that sum to `target`. Returned in reverse order.
///
/// ```
/// use aoc2020::find_sum_factors;
///
/// // n=1
/// assert_eq!(find_sum_factors(1,
///                             &[1721, 979, 366, 299, 675, 1456],
///                             2020),
///            None);
/// assert_eq!(find_sum_factors(1,
///                             &[1721, 979, 366, 299, 675, 1456],
///                             366),
///            Some(vec![366]));
///
/// // n=2
/// assert_eq!(find_sum_factors(2,
///                             &[1721, 979, 366, 299, 675, 1456],
///                             2020),
///            Some(vec![299, 1721]));
/// assert_eq!(find_sum_factors(2,
///                             &[],
///                             2020),
///            None);
/// assert_eq!(find_sum_factors(2,
///                             &[1],
///                             2020),
///            None);
///
/// // n=3
/// assert_eq!(find_sum_factors(3,
///                             &[1721, 979, 366, 299, 675, 1456],
///                             2020),
///            Some(vec![675, 366, 979]));
/// ```
pub fn find_sum_factors(n: u32, xs: &[i32], target: i32) -> Option<Vec<i32>> {
    match n {
        1 => xs.iter().find(|x| **x == target).map(|x| vec![*x]),
        _ => {
            for i in 1..xs.len() {
                let (head, tail) = xs.split_at(i);
                let head : i32 = *head.last().unwrap();
                if let Some(mut v) = find_sum_factors(n-1, tail, target-head) {
                    v.push(head);
                    return Some(v)
                }
            }
            None
        }
    }
}
