use regex::Regex;
use std::collections::HashMap;

enum Rule {
    Char(char),
    List(Vec<u32>),
    Disj(Vec<u32>, Vec<u32>),
}

struct RuleSet {
    rules: HashMap<u32, Rule>,
}

fn parse_rules(rules_string: &str) -> RuleSet {
    let mut rules = HashMap::<u32, Rule>::new();
    for line in rules_string.lines() {
        let mut it = line.split(": ");
        let n: u32 = it.next().unwrap().parse().unwrap();
        let tokens: Vec<_> = it.next().unwrap().split(' ').collect();
        let rule = if tokens.len() == 1 && tokens[0].starts_with('"') {
            Rule::Char(tokens[0].chars().nth(1).unwrap())
        } else if let Some((i, _)) =
            tokens.iter().enumerate().find(|(_, t)| t == &&"|")
        {
            Rule::Disj(
                tokens[..i].iter().map(|t| t.parse().unwrap()).collect(),
                tokens[(i + 1)..]
                    .iter()
                    .map(|t| t.parse().unwrap())
                    .collect(),
            )
        } else {
            Rule::List(tokens.iter().map(|t| t.parse().unwrap()).collect())
        };
        rules.insert(n, rule);
    }
    RuleSet { rules }
}

fn prepend(prefix: &[u32], suffix: &[u32]) -> Vec<u32> {
    let mut v = prefix.to_owned();
    v.extend_from_slice(suffix);
    v
}

impl RuleSet {
    fn matches(&self, rule_list: &[u32], msg: &str) -> bool {
        use Rule::*;
        if rule_list.is_empty() {
            return msg.is_empty();
        }
        let head = rule_list[0];
        let tail = &rule_list[1..];
        let res = match self.rules.get(&head).unwrap() {
            Char(c) => msg.starts_with(*c) && self.matches(tail, &msg[1..]),
            List(l) => self.matches(&prepend(l, &tail), msg),
            Disj(l1, l2) => {
                self.matches(&prepend(l1, tail), msg)
                    || self.matches(&prepend(l2, tail), msg)
            }
        };
        //println!("{:?} matching '{}' => {}", rule_list, msg, res);
        res
    }

    fn to_regex_str(&self, idx: u32) -> String {
        let rule = self.rules.get(&idx).unwrap();
        use Rule::*;
        match rule {
            Char(c) => format!("{}", c),
            List(l) => l.iter().map(|i| self.to_regex_str(*i)).collect(),
            Disj(l1, l2) => format!(
                "({}|{})",
                l1.iter().map(|i| self.to_regex_str(*i)).collect::<String>(),
                l2.iter().map(|i| self.to_regex_str(*i)).collect::<String>()
            ),
        }
    }
}

pub fn part1_regex(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let rules = it.next().unwrap();
    let messages = it.next().unwrap();
    let rule_set = parse_rules(rules);

    let re_str = format!("^{}$", rule_set.to_regex_str(0));
    //println!("Regex '{}'", re_str);
    let re = Regex::new(&re_str).unwrap();
    messages.lines()/*.inspect(|m| println!("'{}' match: {:?}", m, re.is_match(m)))*/.filter(|m| re.is_match(m)).count()
}

pub fn part1(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let rules = it.next().unwrap();
    let messages = it.next().unwrap();
    let rule_set = parse_rules(rules);
    messages
        .lines()
        .filter(|m| rule_set.matches(&[0], m))
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let rules = it.next().unwrap();
    let messages = it.next().unwrap();
    let mut rule_set = parse_rules(rules);
    rule_set.rules.insert(8, Rule::Disj(vec![42], vec![42, 8]));
    rule_set
        .rules
        .insert(11, Rule::Disj(vec![42, 31], vec![42, 11, 31]));
    messages
        .lines()
        .filter(|m| rule_set.matches(&[0], m))
        .count()
}

#[cfg(test)]
#[test]
fn test_part1() {
    let rule_set = parse_rules(
        r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
"#,
    );
    assert!(rule_set.matches(&[0], "ababbb"));
    assert!(rule_set.matches(&[0], "abbbab"));

    assert_eq!(
        part1(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#
        ),
        2
    );

    assert_eq!(
        part2(
            r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
        ),
        12
    );
}
