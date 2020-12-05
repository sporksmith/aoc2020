use std::error::Error;

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
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(seat_num("BFFFBBFRRR").unwrap(), 567);
        assert_eq!(seat_num("FFFBBBFRRR").unwrap(), 119);
        assert_eq!(seat_num("BBFFBBFRLL").unwrap(), 820);

        use std::io::Cursor;
        assert_eq!(
            highest_seat_id(Cursor::new("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"))
                .unwrap(),
            820
        );
    }
}
