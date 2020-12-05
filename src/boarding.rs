use std::error::Error;

#[cfg(test)]
#[test]
fn test_seat_num() {
    assert_eq!(seat_num("BFFFBBFRRR").unwrap(), 567);
    assert_eq!(seat_num("FFFBBBFRRR").unwrap(), 119);
    assert_eq!(seat_num("BBFFBBFRLL").unwrap(), 820);
}

fn seat_num(s: &str) -> Result<usize, std::num::ParseIntError> {
    let s = s.chars().map(|x| match x {
        'F' => '0',
        'B' => '1',
        'L' => '0',
        'R' => '1',
        x => x,
    });
    usize::from_str_radix(s.collect::<String>().as_str(), 2)
}

#[cfg(test)]
#[test]
fn test_highest_seat_id() {
    use std::io::Cursor;
    assert_eq!(
        highest_seat_id(Cursor::new("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"))
            .unwrap(),
        820
    );
}

pub fn highest_seat_id<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let mut result_nums =
        reader.lines().map(|l| -> Result<usize, Box<dyn Error>> {
            Ok(seat_num(l?.as_str())?)
        });
    result_nums
        .try_fold(0, |acc, result_num| Ok(std::cmp::max(acc, result_num?)))
}

#[cfg(test)]
#[test]
fn test_missing_seat_id() {
    use std::io::Cursor;
    assert_eq!(
        // 117, 567, 119, 820
        missing_seat_id(Cursor::new(
            "FFFBBBFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"
        ))
        .unwrap(),
        118
    );
}

pub fn missing_seat_id<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let nums: Result<Vec<usize>, Box<dyn Error>> = reader
        .lines()
        .map(|l| -> Result<usize, Box<dyn Error>> {
            Ok(seat_num(l?.as_str())?)
        })
        .collect();
    let mut nums = nums?;
    nums.sort_unstable();
    for (num, next) in nums.iter().zip(nums.iter().skip(1)) {
        if *next == num + 2 {
            return Ok(num + 1);
        }
    }
    Err("Not found".into())
}
