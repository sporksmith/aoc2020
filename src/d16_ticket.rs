use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Debug)]
pub struct Rule {
    pub field: String,
    pub ranges: [RangeInclusive<u64>; 2],
}

impl From<&str> for Rule {
    /// ```
    /// use aoc2020::d16_ticket::*;
    /// assert_eq!(Rule::from("class: 1-3 or 5-7"),
    ///            Rule{ field: "class".into(), ranges: [1..=3, 5..=7]});
    /// ```
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(.*+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Rule {
            field: caps.get(1).unwrap().as_str().into(),
            ranges: [
                caps.get(2).unwrap().as_str().parse().unwrap()
                    ..=caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap()
                    ..=caps.get(5).unwrap().as_str().parse().unwrap(),
            ],
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let rules: Vec<Rule> =
        sections.next().unwrap().lines().map(Rule::from).collect();
    let _mine: Vec<u64> = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let others: Vec<Vec<u64>> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    /*
    others.iter()
        .filter(| ticket |
                ticket.iter().all(
                    | field |
                    rules.iter().any(| rule |
                                     rule.ranges.iter().any(
                                         | range | range.contains(field)))))
        .inspect(| fields | println!("invalid: {:?}", fields))
        .map(| fields | fields.iter().sum::<u64>())
        .sum()
    */
    others
        .iter()
        .flatten()
        .filter(|field| {
            !rules.iter().any(|rule| {
                rule.ranges.iter().any(|range| range.contains(field))
            })
        })
        .sum()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(part1(input), 71);
}
