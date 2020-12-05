use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::BufRead;

/// ```
/// use aoc2020::passport::parse_key_val;
/// assert_eq!(parse_key_val("key:val"), ("key", "val"));
/// assert_eq!(parse_key_val("key:"), ("key", ""));
/// assert_eq!(parse_key_val("key"), ("key", ""));
/// assert_eq!(parse_key_val(""), ("", ""));
/// ```
pub fn parse_key_val(s: &str) -> (&str, &str) {
    let mut key_val_seq = s.splitn(2, ':');
    // XXX: Yuck.
    let key = key_val_seq.next().unwrap_or("");
    let val = key_val_seq.next().unwrap_or("");
    assert!(key_val_seq.next().is_none());
    (key, val)
}

pub struct BufReadSplitOnBlank<R: BufRead> {
    lines: std::io::Lines<R>,
    done: bool,
}

impl<R: BufRead> BufReadSplitOnBlank<R> {
    pub fn new(reader: R) -> BufReadSplitOnBlank<R> {
        BufReadSplitOnBlank {
            lines: reader.lines(),
            done: false,
        }
    }
}

/// Adapt a `BufRead` to return chunks separated by blank lines.
/// Gnarly; would definitely like to see a cleaner way of doing this.
/// ```
/// use aoc2020::passport::*;
/// use std::io::Cursor;
/// let input = "\
/// line1
/// line2
///
/// line3
/// line4";
/// let reader = Cursor::new(input.as_bytes());
/// let result : Vec<_> = BufReadSplitOnBlank::new(reader).map(|x| x.unwrap()).collect();
/// let expected_result : Vec<Vec<String>> = vec![
///            vec!["line1".to_string(), "line2".to_string()],
///            vec!["line3".to_string(), "line4".to_string()]];
/// assert_eq!(result, expected_result);
/// ```
impl<R: BufRead> Iterator for BufReadSplitOnBlank<R> {
    type Item = Result<Vec<String>, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut rv = Vec::<String>::new();
        loop {
            let line = match self.lines.next() {
                None => {
                    self.done = true;
                    break;
                }
                Some(Err(e)) => return Some(Err(e.into())),
                Some(Ok(l)) => l,
            };
            if line == "" {
                break;
            }
            rv.push(line);
        }
        if self.done && rv.is_empty() {
            None
        } else {
            Some(Ok(rv))
        }
    }
}

/// Represent a passport. For this part we don't actually need to store anything yet.
pub struct Passport {}

/// Trait to create a passport from a set of key/value pairs.
impl TryFrom<HashMap<String, String>> for Passport {
    type Error = String;
    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        for key in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
            if !value.contains_key(*key) {
                return Err(format!("Missing key {}", key));
            }
        }
        Ok(Passport {})
    }
}

/// Stricter passport, for part 2.
pub struct StrictPassport {}

fn parse_int_field(
    name: &str,
    s: &str,
    min: i32,
    max: i32,
) -> Result<i32, Box<dyn Error>> {
    let i: i32 = s.parse()?;
    if !(i >= min && i <= max) {
        return Err(format!("{}={} out of range", name, i).into());
    }
    Ok(i)
}

/// Trait to create a passport from a set of key/value pairs.
impl TryFrom<HashMap<String, String>> for StrictPassport {
    type Error = Box<dyn Error>;
    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        parse_int_field(
            "byr",
            value.get("byr").ok_or_else(|| "Missing byr")?,
            1920,
            2002,
        )?;
        parse_int_field(
            "iyr",
            value.get("iyr").ok_or_else(|| "Missing iyr")?,
            2010,
            2020,
        )?;
        parse_int_field(
            "eyr",
            value.get("eyr").ok_or_else(|| "Missing eyr")?,
            2020,
            2030,
        )?;

        // Check hgt
        {
            let hgt = value.get("hgt").ok_or_else(|| "Missing hgt")?;
            if hgt.len() < 2 {
                return Err(format!("Bad hgt {}", hgt).into());
            }
            let (value, unit) = hgt.split_at(hgt.len() - 2);
            let value: i32 = value.parse()?;
            let range = match unit {
                "in" => (59..=76),
                "cm" => (150..=193),
                _ => return Err(format!("hgt bad unit {}", unit).into()),
            };
            if !range.contains(&value) {
                return Err(
                    format!("hgt {}{} out of range", value, unit).into()
                );
            }
        }

        // Check hcl
        {
            let hcl = value.get("hcl").ok_or_else(|| "Missing hcl")?;
            if !(hcl.starts_with('#')
                && hcl.len() == 6
                && hcl.chars().all(|c| c.is_digit(16)))
            {
                return Err(format!("bad hcl: {}", hcl).into());
            }
        }

        // Check ecl
        {
            let ecl = value.get("ecl").ok_or_else(|| "Missing ecl")?;
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&ecl.as_str())
            {
                return Err(format!("bad ecl: {}", ecl).into());
            }
        }

        // Check pid
        {
            let pid = value.get("pid").ok_or_else(|| "Missing pid")?;
            if !(pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))) {
                return Err(format!("bad pid: {}", pid).into());
            }
        }

        Ok(StrictPassport {})
    }
}

/// ```
/// use std::io::Cursor;
/// use aoc2020::passport::*;
/// let input = "\
///   ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
///   byr:1937 iyr:2017 cid:147 hgt:183cm
///
///   iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
///   hcl:#cfa07d byr:1929
///
///   hcl:#ae17e1 iyr:2013
///   eyr:2024
///   ecl:brn pid:760753108 byr:1931
///   hgt:179cm
///
///   hcl:#cfa07d eyr:2025 pid:166559648
///   iyr:2011 ecl:brn hgt:59in
/// ";
/// assert_eq!(count_valid_passports::<Passport, _>(Cursor::new(input.as_bytes())).unwrap(), 2);
/// ```
pub fn count_valid_passports<
    P: TryFrom<HashMap<String, String>>,
    R: std::io::BufRead,
>(
    input: R,
) -> Result<usize, Box<dyn Error>> {
    // Try converting each chunk of text (separated by blank lines) into a passport, counting
    // the number of valid conversions.
    BufReadSplitOnBlank::new(input).try_fold(
        0,
        |acc, lines| -> Result<usize, Box<dyn Error>> {
            // Bubble up IO errors.
            let lines = lines?;

            // Convert vector of lines into a flat iterator over tokens
            let tokens =
                lines.iter().map(|l| l.split_ascii_whitespace()).flatten();

            // Convert tokens to a hashmap
            let m: HashMap<String, String> = tokens
                .map(parse_key_val)
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            // Add to running total iff can be converted into a passport.
            Ok(acc
                + match P::try_from(m) {
                    Ok(_) => 1,
                    Err(_) => 0,
                })
        },
    )
}
