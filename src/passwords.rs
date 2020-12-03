use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

/// Since we end up implementing multiple policies, and could end up needing more in later days, we
/// implement a generic policy trait here.
pub trait PasswordPolicy {
    /// Returns whether `password` satisfies the policy.
    fn check(&self, password: &str) -> bool;
}

/// The part-1 policy.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LegacyPasswordPolicy {
    /// Min occurrences of `c`
    pub min: usize,
    /// Max occurrences of `c`
    pub max: usize,
    pub c: char,
}

/// Implement `FromStr` to idiomatically make it parseable.
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

/// Implement the part-1 policy.
impl PasswordPolicy for LegacyPasswordPolicy {
    /// Returns whether `password` has at least `min` and at most `max` occurrences of `c`.
    ///
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

/// The part-2 policy.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NewPasswordPolicy {
    /// 1-indexed position
    pub pos1: usize,
    /// 1-indexed position
    pub pos2: usize,
    pub c: char,
}

/// Make this one parseable as well.
impl FromStr for NewPasswordPolicy {
    type Err = Box<dyn Error>;
    /// ```
    /// use aoc2020::passwords::NewPasswordPolicy;
    /// assert_eq!("1-3 a".parse::<NewPasswordPolicy>().unwrap(), NewPasswordPolicy{pos1: 1, pos2: 3, c: 'a'});
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap();
        }
        let captures = RE.captures(s).ok_or("Regex match")?;
        let pos1: usize = captures.get(1).ok_or("Missing min")?.as_str().parse()?;
        let pos2: usize = captures.get(2).ok_or("Missing max")?.as_str().parse()?;
        let c: char = captures.get(3).ok_or("Missing char")?.as_str().parse()?;
        Ok(NewPasswordPolicy { pos1, pos2, c })
    }
}

/// Implement the generic policy trait.
impl PasswordPolicy for NewPasswordPolicy {
    /// Returns whether exactly one of the policy's positions in `password` has the
    /// policy-character `c`.
    ///
    /// ```
    /// use aoc2020::passwords::{PasswordPolicy, NewPasswordPolicy};
    /// // Examples from problem statement:
    /// assert!(NewPasswordPolicy{pos1: 1, pos2: 3, c: 'a'}.check("abcde"));
    /// assert!(!NewPasswordPolicy{pos1: 1, pos2: 3, c: 'b'}.check("cdefg"));
    /// assert!(!NewPasswordPolicy{pos1: 2, pos2: 9, c: 'c'}.check("ccccccccc"));
    /// ```
    fn check(&self, password: &str) -> bool {
        let check_pos = |p: usize| password.chars().nth(p - 1).unwrap() == self.c;
        let p1 = check_pos(self.pos1);
        let p2 = check_pos(self.pos2);
        (p1 || p2) && !(p1 && p2)
    }
}

/// Parse a line of input into a `PasswordPolicy` and password string.
///
/// ```
/// use aoc2020::passwords::{LegacyPasswordPolicy, NewPasswordPolicy, parse_line};
/// assert_eq!(parse_line("1-3 a: abcde").unwrap(), (LegacyPasswordPolicy{min:1, max:3, c: 'a'}, "abcde"));
/// assert_eq!(parse_line("1-3 a: abcde").unwrap(), (NewPasswordPolicy{pos1:1, pos2:3, c: 'a'}, "abcde"));
/// ```
pub fn parse_line<P>(s: &str) -> Result<(P, &str), Box<dyn Error>>
where
    // XXX: I'd prefer to not force `Err` to be *exactly* `Box<dyn Error>`, but instead just something
    // implementing the `Error` trait (making it convertible to `Box<dyn Error>`, or even more
    // generally just something convertible to `Box<dyn Error>`. Couldn't quite get such a thing to
    // compile though; the compiler suggested adding a `'static` lifetime bound to the error type to
    // resolve it,but that doesn't seem right either.
    P: FromStr<Err = Box<dyn Error>> + PasswordPolicy,
{
    let mut parts = s.splitn(2, ": ");
    let policy = parts.next().ok_or("Missing policy")?.parse::<P>()?;
    let password = parts.next().ok_or("Missing password")?;
    Ok((policy, password))
}

/// Parse a line of input, containing a policy and password, and return whether the password
/// satisfies the policy.
///
/// ```
/// use aoc2020::passwords::{check_line, LegacyPasswordPolicy};
/// assert!(check_line::<LegacyPasswordPolicy>("1-3 a: abcde").unwrap());
/// assert!(!check_line::<LegacyPasswordPolicy>("1-3 b: cdefg").unwrap());
/// assert!(check_line::<LegacyPasswordPolicy>("2-9 c: ccccccccc").unwrap());
/// ```
/// XXX: as above re would be nice to be more permissive about `FromStr`'s `Err` type.
///
/// XXX: Originally I tried having this take an `Iterator<Item=&str>` to push the iteration here
/// where it's testable. I couldn't figure out a way to get such an iterator out of `stdin` without
/// reading and storing the whole `stdin` at once, though. (In practice doing that would be fine
/// for the given test inputs, but I try to make a habit of not storing an entire file/input of
/// unknown size when streaming/iterating over it will do).
pub fn check_line<P>(line: &str) -> Result<bool, Box<dyn Error>>
where
    P: FromStr<Err = Box<dyn Error>> + PasswordPolicy,
{
    let (policy, password) = parse_line::<P>(line)?;
    Ok(policy.check(password))
}
