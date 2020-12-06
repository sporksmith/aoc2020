use aoc2020::BufReadSplitOnBlank;
use std::collections::HashSet;
use std::error::Error;

fn unique_char_count<It: Iterator<Item = char>>(it: It) -> usize {
    it.fold(HashSet::new(), |mut acc, c| {
        acc.insert(c);
        acc
    })
    .len()
}

#[cfg(test)]
#[test]
fn test_unique_char_count() {
    assert_eq!(unique_char_count("abc".chars()), 3);
    assert_eq!(unique_char_count("aba".chars()), 2);
    assert_eq!(unique_char_count("aa".chars()), 1);
}

fn sum_of_unique_answers<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let groups = BufReadSplitOnBlank::new(reader);
    Ok(groups
        .map(|res| {
            Ok(unique_char_count(res?.iter().map(|l| l.chars()).flatten()))
        })
        // XXX: Is there some way to reuse `sum` here, with an adapter
        // to try unwrapping the operand, and to wrap the result?
        .try_fold::<_, _, std::result::Result<usize, Box<dyn Error>>>(
            0,
            |sum, count: std::result::Result<_, Box<dyn Error>>| {
                Ok(sum + count?)
            },
        )?)
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
