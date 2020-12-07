use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct BagColor(String);

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    outer: BagColor,
    inner: Vec<(usize, BagColor)>,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
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
        for s in inner_string.split(", ") {
            // trim potential 's' in "bags"
            let s = s.trim_end_matches('s');

            // trim "bag"
            let s = s.trim_end_matches(" bag");

            let mut n_and_color = s.splitn(2, ' ');
            let n = n_and_color.next().ok_or("No n")?.parse()?;
            let color = BagColor(n_and_color.next().ok_or("No color")?.into());
            inner.push((n, color));
        }
        Ok(Rule { outer, inner })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_rules() {
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
    }
}

fn main() {
    unimplemented!();
}
