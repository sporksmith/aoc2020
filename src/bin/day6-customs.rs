use aoc2020::BufReadSplitOnBlank;
use std::collections::HashSet;
use std::error::Error;

fn sum_of_unique_answers<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    let groups = BufReadSplitOnBlank::new(reader);
    for res in groups {
        let lines = res?;
        let mut hs = HashSet::new();
        for c in lines.iter().map(|l| l.chars()).flatten() {
            hs.insert(c);
        }
        sum += hs.len();
    }
    Ok(sum)
}

#[cfg(test)]
#[test]
fn test_sum_of_unique_answers() {
    use std::io::Cursor;
    let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(
        sum_of_unique_answers(Cursor::new(input.as_bytes())).unwrap(),
        11
    );
}

fn main() {
    let part = std::env::args().nth(1).expect("missing part");
    let fun = match part.as_str() {
        "a" => sum_of_unique_answers,
        //"b" => missing_seat_id,
        _ => panic!("Bad part {}", part),
    };
    println!("{}", fun(std::io::stdin().lock()).unwrap());
}
