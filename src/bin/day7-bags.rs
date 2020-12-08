use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;

// TODO: On large input, could save substantial memory and comparison/hashing
// by interning the color strings.
#[derive(Debug, PartialEq, Eq, Hash)]
struct BagColor(String);

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    outer: BagColor,
    inner: Vec<(usize, BagColor)>,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;
    /// Parse a single line, such as:
    ///
    /// light red bags contain 1 bright white bag, 2 muted yellow bags.
    /// bright white bags contain 1 shiny gold bag.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut contains_split = s.splitn(2, " bags contain ");
        let outer = BagColor(
            contains_split
                .next()
                .ok_or("No outer bag color")?
                .to_string(),
        );
        let inner_string = contains_split
            .next()
            .ok_or("No bag color list")?
            .trim_end_matches('.');
        assert!(contains_split.next().is_none());

        let mut inner = Vec::<(usize, BagColor)>::new();
        if !inner_string.starts_with("no other") {
            for s in inner_string.split(", ") {
                // trim potential 's' in "bags"
                let s = s.trim_end_matches('s');

                // trim "bag"
                let s = s.trim_end_matches(" bag");

                let mut n_and_color = s.splitn(2, ' ');
                let n = n_and_color.next().ok_or("No n")?.parse()?;
                let color =
                    BagColor(n_and_color.next().ok_or("No color")?.into());
                inner.push((n, color));
            }
        }
        Ok(Rule { outer, inner })
    }
}

/// Parse a stream (such as stdin) into a list of rules.
fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Rule>, Box<dyn Error>> {
    let mut result = Vec::<Rule>::new();
    for line in reader.lines() {
        result.push(line?.parse()?);
    }
    Ok(result)
}

fn build_inner_to_outer_map(
    rules: &[Rule],
) -> HashMap<&BagColor, HashSet<&BagColor>> {
    let mut result = HashMap::new();
    for rule in rules {
        for inner in &rule.inner {
            result
                .entry(&inner.1)
                .or_insert_with(HashSet::new)
                .insert(&rule.outer);
        }
    }
    result
}

fn number_of_outer_bags_that_could_have_shiny(rules: &[Rule]) -> usize {
    let inner_to_outer_map = build_inner_to_outer_map(&rules);
    let start_point = BagColor("shiny gold".into());
    let mut to_visit = vec![&start_point];
    let mut visited = HashSet::<&BagColor>::new();
    // Recursively find bags that can contain a shiny gold bag.
    while let Some(c) = to_visit.pop() {
        // Process the bag colors that can directly contain the current bag color.
        if let Some(possible_outers) = inner_to_outer_map.get(&c) {
            for outer in possible_outers {
                // Add to set of visited bag colors (which is also the solution set).
                if visited.insert(*outer) {
                    // Since it wasn't already in the solution set, we need
                    // to visit it as well - i.e. find which bag colors can
                    // directly contain this one.
                    to_visit.push(outer);
                }
            }
        }
    }
    visited.len()
}

fn number_of_bags_in_bag_helper<'a>(
    rules_map: &HashMap<&BagColor, &'a Rule>,
    answer_map: &mut HashMap<&'a BagColor, usize>,
    color: &'a BagColor,
) -> usize {
    if let Some(u) = answer_map.get(color) {
        return *u;
    }
    // A missing rule could be reasonably be interpreted
    // as that color being allowed 0 bags, but unless we see that case I'd rather
    // panic here and double-check that the input really is missing the rule.
    let rule = rules_map.get(color).expect("Missing rule");

    let sum = rule
        .inner
        .iter()
        .map(|(n, color)| {
            *n + *n * number_of_bags_in_bag_helper(rules_map, answer_map, color)
        })
        .sum();
    answer_map.insert(color, sum);
    sum
}

fn number_of_bags_in_shiny(rules: &[Rule]) -> usize {
    let mut rules_map = HashMap::<&BagColor, &Rule>::new();
    for rule in rules {
        rules_map.insert(&rule.outer, rule);
    }
    // Drop mutability
    let rules_map = rules_map;

    // Memo-table
    let mut answer_map = HashMap::new();

    number_of_bags_in_bag_helper(
        &rules_map,
        &mut answer_map,
        &BagColor("shiny gold".into()),
    )
}

fn main() {
    let rules = parse_input(std::io::stdin().lock()).unwrap();
    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "a" => number_of_outer_bags_that_could_have_shiny(&rules),
        "b" => number_of_bags_in_shiny(&rules),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_from_str() {
        assert_eq!(
            Rule::from_str(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap(),
            Rule{outer: BagColor("light red".into()),
                 inner: vec![(1, BagColor("bright white".into())),
                             (2, BagColor("muted yellow".into()))]});
        assert_eq!(
            Rule::from_str("bright white bags contain 1 shiny gold bag.")
                .unwrap(),
            Rule {
                outer: BagColor("bright white".into()),
                inner: vec![(1, BagColor("shiny gold".into()))]
            }
        );
        assert_eq!(
            Rule::from_str("bright white bags contain no other bags.").unwrap(),
            Rule {
                outer: BagColor("bright white".into()),
                inner: vec![]
            }
        );
    }

    #[test]
    fn test_parse_input() {
        use std::io::Cursor;
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.";
        assert_eq!(
            parse_input(Cursor::new(input.as_bytes())).unwrap(),
            vec![
                Rule {
                    outer: BagColor("light red".into()),
                    inner: vec![
                        (1, BagColor("bright white".into())),
                        (2, BagColor("muted yellow".into()))
                    ]
                },
                Rule {
                    outer: BagColor("bright white".into()),
                    inner: vec![(1, BagColor("shiny gold".into()))]
                }
            ]
        );
    }

    #[test]
    fn test_number_of_outer_bags_that_could_have_shiny() {
        let mut rules = Vec::<Rule>::new();
        assert_eq!(number_of_outer_bags_that_could_have_shiny(&rules), 0);

        // Add a color that can directly contain shiny gold.
        rules.push(Rule {
            outer: BagColor("direct".into()),
            inner: vec![(1, BagColor("shiny gold".into()))],
        });
        assert_eq!(number_of_outer_bags_that_could_have_shiny(&rules), 1);

        // Add a color that can contain one that can contain shiny gold.
        rules.push(Rule {
            outer: BagColor("indirect".into()),
            inner: vec![(1, BagColor("direct".into()))],
        });
        assert_eq!(number_of_outer_bags_that_could_have_shiny(&rules), 2);

        // Add a cycle.
        rules.push(Rule {
            outer: BagColor("direct".into()),
            inner: vec![(1, BagColor("indirect".into()))],
        });
        assert_eq!(number_of_outer_bags_that_could_have_shiny(&rules), 2);
    }

    #[test]
    fn test_number_of_bags_in_shiny() {
        let rules = vec![Rule {
            outer: BagColor("shiny gold".into()),
            inner: vec![],
        }];
        assert_eq!(number_of_bags_in_shiny(&rules), 0);

        let rules = vec![
            Rule {
                outer: BagColor("shiny gold".into()),
                inner: vec![(2, BagColor("gray".into()))],
            },
            Rule {
                outer: BagColor("gray".into()),
                inner: vec![],
            },
        ];
        assert_eq!(number_of_bags_in_shiny(&rules), 2);
    }

    #[test]
    fn test_sample_input() {
        use std::io::Cursor;
        let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let rules = parse_input(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(number_of_outer_bags_that_could_have_shiny(&rules), 4);
        assert_eq!(number_of_bags_in_shiny(&rules), 32);

        let input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let rules = parse_input(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(number_of_bags_in_shiny(&rules), 126);
    }
}
