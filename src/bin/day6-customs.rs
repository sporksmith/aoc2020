use aoc2020::BufReadSplitOnBlank;
use std::error::Error;
use std::collections::HashSet;

fn unique_answers<'a, I: Iterator<Item=&'a String>>(
    strings: I,
) -> usize {
    strings
        .fold(HashSet::<char>::new (), | mut acc, s |
                  {
                      s.chars().for_each(| c | {acc.insert(c);});
                      acc
                  })
        .len()
}

fn sum_of_unique_answers<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    let groups = BufReadSplitOnBlank::new(reader);
    for res in groups {
        sum += unique_answers(res?.iter());
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
        sum_of_unique_answers(Cursor::new(input.as_bytes()))
            .unwrap(),
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
