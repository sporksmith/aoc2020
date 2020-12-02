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
    /// ```
    pub fn check(&self, password: &str) -> bool {
        let c = password.chars().filter(|c| c == &self.c).count();
        c >= self.min && c <= self.max
    }
}
