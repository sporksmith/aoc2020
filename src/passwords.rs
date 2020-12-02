use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PasswordPolicy {
    pub min: usize,
    pub max: usize,
    pub c: char,
}

impl FromStr for PasswordPolicy {
    type Err = Box<dyn Error>;
    /// ```
    /// use aoc2020::passwords::PasswordPolicy;
    /// assert_eq!("1-3 a".parse::<PasswordPolicy>().unwrap(), PasswordPolicy{min: 1, max: 3, c: 'a'});
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();
        }
        let captures = RE.captures(s).ok_or("Regex match")?;
        let min: usize = captures.get(1).ok_or("Missing min")?.as_str().parse()?;
        let max: usize = captures.get(2).ok_or("Missing max")?.as_str().parse()?;
        let c: char = captures.get(3).ok_or("Missing max")?.as_str().parse()?;
        Ok(PasswordPolicy { min, max, c })
    }
}

impl PasswordPolicy {
    /// ```
    /// use aoc2020::passwords::PasswordPolicy;
    /// assert!(!PasswordPolicy{min: 1, max: 2, c: 'a'}.check("bbb"));
    /// assert!(PasswordPolicy{min: 1, max: 2, c: 'a'}.check("babb"));
    /// assert!(PasswordPolicy{min: 1, max: 2, c: 'a'}.check("babab"));
    /// assert!(!PasswordPolicy{min: 1, max: 2, c: 'a'}.check("bababa"));
    ///
    /// // Examples from problem statement:
    /// assert!(PasswordPolicy{min: 1, max: 3, c: 'a'}.check("abcde"));
    /// assert!(!PasswordPolicy{min: 1, max: 3, c: 'b'}.check("cdefg"));
    /// assert!(PasswordPolicy{min: 2, max: 9, c: 'c'}.check("ccccccccc"));
    /// ```
    pub fn check(&self, password: &str) -> bool {
        let c = password.chars().filter(|c| c == &self.c).count();
        c >= self.min && c <= self.max
    }
}

/// ```
/// use aoc2020::passwords::{PasswordPolicy, parse_line};
/// assert_eq!(parse_line("1-3 a: abcde").unwrap(), (PasswordPolicy{min:1, max:3, c: 'a'}, "abcde"));
/// ```
pub fn parse_line(s: &str) -> Result<(PasswordPolicy, &str), Box<dyn Error>> {
    let mut parts = s.splitn(2, ": ");
    let policy = parts
        .next()
        .ok_or("Missing policy")?
        .parse::<PasswordPolicy>()?;
    let password = parts.next().ok_or("Missing password")?;
    Ok((policy, password))
}

/// ```
/// use aoc2020::passwords::count_valid;
/// assert_eq!(count_valid(["1-3 a: abcde","1-3 b: cdefg", "2-9 c: ccccccccc"].iter().copied()).unwrap(), 2);
/// ```
pub fn count_valid<'a, It: Iterator<Item = &'a str>>(lines: It) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for line in lines {
        let (policy, password) = parse_line(line)?;
        if policy.check(password) {
            count += 1;
        }
    }
    Ok(count)
}
