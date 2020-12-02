use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

pub trait PasswordPolicy {
    fn check(&self, password: &str) -> bool;
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LegacyPasswordPolicy {
    pub min: usize,
    pub max: usize,
    pub c: char,
}

impl FromStr for LegacyPasswordPolicy {
    type Err = Box<dyn Error>;
    /// ```
    /// use aoc2020::passwords::LegacyPasswordPolicy;
    /// assert_eq!("1-3 a".parse::<LegacyPasswordPolicy>().unwrap(), LegacyPasswordPolicy{min: 1, max: 3, c: 'a'});
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();
        }
        let captures = RE.captures(s).ok_or("Regex match")?;
        let min: usize = captures.get(1).ok_or("Missing min")?.as_str().parse()?;
        let max: usize = captures.get(2).ok_or("Missing max")?.as_str().parse()?;
        let c: char = captures.get(3).ok_or("Missing max")?.as_str().parse()?;
        Ok(LegacyPasswordPolicy { min, max, c })
    }
}

impl PasswordPolicy for LegacyPasswordPolicy {
    /// ```
    /// use aoc2020::passwords::{PasswordPolicy, LegacyPasswordPolicy};
    /// assert!(!LegacyPasswordPolicy{min: 1, max: 2, c: 'a'}.check("bbb"));
    /// assert!(LegacyPasswordPolicy{min: 1, max: 2, c: 'a'}.check("babb"));
    /// assert!(LegacyPasswordPolicy{min: 1, max: 2, c: 'a'}.check("babab"));
    /// assert!(!LegacyPasswordPolicy{min: 1, max: 2, c: 'a'}.check("bababa"));
    ///
    /// // Examples from problem statement:
    /// assert!(LegacyPasswordPolicy{min: 1, max: 3, c: 'a'}.check("abcde"));
    /// assert!(!LegacyPasswordPolicy{min: 1, max: 3, c: 'b'}.check("cdefg"));
    /// assert!(LegacyPasswordPolicy{min: 2, max: 9, c: 'c'}.check("ccccccccc"));
    /// ```
    fn check(&self, password: &str) -> bool {
        let c = password.chars().filter(|c| c == &self.c).count();
        c >= self.min && c <= self.max
    }
}

/// ```
/// use aoc2020::passwords::{LegacyPasswordPolicy, parse_line};
/// assert_eq!(parse_line("1-3 a: abcde").unwrap(), (LegacyPasswordPolicy{min:1, max:3, c: 'a'}, "abcde"));
/// ```
pub fn parse_line<P>(s: &str) -> Result<(P, &str), Box<dyn Error>>
where
    P: FromStr<Err = std::boxed::Box<dyn std::error::Error>> + PasswordPolicy,
{
    let mut parts = s.splitn(2, ": ");
    let policy = parts.next().ok_or("Missing policy")?.parse::<P>()?;
    let password = parts.next().ok_or("Missing password")?;
    Ok((policy, password))
}

/// ```
/// use aoc2020::passwords::{check_line, LegacyPasswordPolicy};
/// assert!(check_line::<LegacyPasswordPolicy>("1-3 a: abcde").unwrap());
/// assert!(!check_line::<LegacyPasswordPolicy>("1-3 b: cdefg").unwrap());
/// assert!(check_line::<LegacyPasswordPolicy>("2-9 c: ccccccccc").unwrap());
/// ```
pub fn check_line<P>(line: &str) -> Result<bool, Box<dyn Error>>
where
    P: FromStr<Err = std::boxed::Box<dyn std::error::Error>> + PasswordPolicy,
{
    let (policy, password) = parse_line::<P>(line)?;
    Ok(policy.check(password))
}
