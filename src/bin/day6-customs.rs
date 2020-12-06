use aoc2020::BufReadSplitOnBlank;
use std::collections::HashSet;
use std::error::Error;

#[cfg(test)]
#[test]
fn test_unique_answer_count() {
    assert_eq!(unique_answer_count(["abc"].iter().copied()), 3);
    assert_eq!(unique_answer_count(["a", "b", "c"].iter().copied()), 3);
    assert_eq!(unique_answer_count(["ab", "ac"].iter().copied()), 3);
}

fn unique_answer_count<'a, It: Iterator<Item = &'a str>>(it: It) -> usize {
    it.map(|s| s.chars())
        .flatten()
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c);
            acc
        })
        .len()
}

#[cfg(test)]
#[test]
fn test_unanimous_answer_count() {
    assert_eq!(unanimous_answer_count(["abc"].iter().copied()), 3);
    assert_eq!(unanimous_answer_count(["a", "b", "c"].iter().copied()), 0);
    assert_eq!(unanimous_answer_count(["ab", "ac"].iter().copied()), 1);
}

#[allow(dead_code)]
fn unanimous_answer_count<'a, It: Iterator<Item = &'a str>>(it: It) -> usize {
    let mut sets = it.map(|line| {
        let mut set = HashSet::new();
        line.chars().for_each(|c| {
            set.insert(c);
        });
        set
    });
    if let Some(first_set) = sets.next() {
        sets.fold(first_set, |acc, s| acc.intersection(&s).copied().collect())
            .len()
    } else {
        0
    }
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

fn sum_of_unique_answers<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let groups = BufReadSplitOnBlank::new(reader);
    Ok(groups
        .map(|res| Ok(unique_answer_count(res?.iter().map(|l| l.as_str()))))
        .try_fold::<_, _, std::result::Result<usize, Box<dyn Error>>>(
            0,
            |sum, count: std::result::Result<_, Box<dyn Error>>| {
                Ok(sum + count?)
            },
        )?)
}

// XXX: Can we deduplicate with sum_of_unique_answers?
fn sum_of_unanimous_answers<R: std::io::BufRead>(
    reader: R,
) -> Result<usize, Box<dyn Error>> {
    let groups = BufReadSplitOnBlank::new(reader);
    Ok(groups
        .map(|res| Ok(unanimous_answer_count(res?.iter().map(|l| l.as_str()))))
        .try_fold::<_, _, std::result::Result<usize, Box<dyn Error>>>(
            0,
            |sum, count: std::result::Result<_, Box<dyn Error>>| {
                Ok(sum + count?)
            },
        )?)
}

fn main() {
    let part = std::env::args().nth(1).expect("missing part");
    let fun = match part.as_str() {
        "a" => sum_of_unique_answers,
        "b" => sum_of_unanimous_answers,
        _ => panic!("Bad part {}", part),
    };
    println!("{}", fun(std::io::stdin().lock()).unwrap());
}
