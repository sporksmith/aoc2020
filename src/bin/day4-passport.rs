use aoc2020::BufReadSplitOnBlank;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;

pub fn parse_key_val(s: &str) -> (&str, &str) {
    let mut key_val_seq = s.splitn(2, ':');
    // XXX: Yuck.
    let key = key_val_seq.next().unwrap_or("");
    let val = key_val_seq.next().unwrap_or("");
    assert!(key_val_seq.next().is_none());
    (key, val)
}

#[cfg(test)]
#[test]
fn test_parse_key_val() {
    assert_eq!(parse_key_val("key:val"), ("key", "val"));
    assert_eq!(parse_key_val("key:"), ("key", ""));
    assert_eq!(parse_key_val("key"), ("key", ""));
    assert_eq!(parse_key_val(""), ("", ""));
}

pub fn key_val_lines_to_hashmap<'a, I: Iterator<Item = &'a str>>(
    it: I,
) -> HashMap<String, String> {
    // Convert vector of lines into a flat iterator over tokens
    let tokens = it.map(|l| l.split_ascii_whitespace()).flatten();

    // Convert tokens to a hashmap
    tokens
        .map(parse_key_val)
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[cfg(test)]
#[test]
fn test_parse_key_val_lines_to_hashmap() {
    assert_eq!(
        key_val_lines_to_hashmap(["k1:v1 k2:v2", "k3:v3"].iter().copied()),
        [
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
            ("k3".to_string(), "v3".to_string())
        ]
        .iter()
        .cloned()
        .collect()
    );
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
            value.get("byr").ok_or("Missing byr")?,
            1920,
            2002,
        )?;
        parse_int_field(
            "iyr",
            value.get("iyr").ok_or("Missing iyr")?,
            2010,
            2020,
        )?;
        parse_int_field(
            "eyr",
            value.get("eyr").ok_or("Missing eyr")?,
            2020,
            2030,
        )?;

        // Check hgt
        {
            let hgt = value.get("hgt").ok_or("Missing hgt")?;
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
            let hcl = value.get("hcl").ok_or("Missing hcl")?;
            if !(hcl.starts_with('#')) {
                return Err(format!("hcl missing #: {}", hcl).into());
            }
            let (_, val) = hcl.split_at(1);
            if !(val.len() == 6 && val.chars().all(|c| c.is_digit(16))) {
                return Err(format!("bad hcl val: {}", val).into());
            }
        }

        // Check ecl
        {
            let ecl = value.get("ecl").ok_or("Missing ecl")?;
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .contains(&ecl.as_str())
            {
                return Err(format!("bad ecl: {}", ecl).into());
            }
        }

        // Check pid
        {
            let pid = value.get("pid").ok_or("Missing pid")?;
            if !(pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))) {
                return Err(format!("bad pid: {}", pid).into());
            }
        }

        Ok(StrictPassport {})
    }
}

#[cfg(test)]
#[test]
fn test_parse_strict_passport() {
    let parse = |s| {
        StrictPassport::try_from(key_val_lines_to_hashmap([s].iter().copied()))
    };
    parse(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
    )
    .unwrap();
    parse("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm").unwrap();
    parse("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022").unwrap();
    parse(
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    )
    .unwrap();

    assert!(parse("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926").is_err());
    assert!(parse("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
    )
    .is_err());
    assert!(parse("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277").is_err());
    assert!(parse(
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
    )
    .is_err());
}

#[cfg(test)]
#[test]
fn test_count_valid_passports() {
    use std::io::Cursor;
    let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
 ";
    assert_eq!(
        count_valid_passports::<Passport, _>(Cursor::new(input.as_bytes()))
            .unwrap(),
        2
    );
}

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
            let map = key_val_lines_to_hashmap(tokens);

            // Add to running total iff can be converted into a passport.
            Ok(acc
                + match P::try_from(map) {
                    Ok(_) => 1,
                    Err(_) => 0,
                })
        },
    )
}

fn main() {
    let part = std::env::args().nth(1).expect("missing part");
    let fun = match part.as_str() {
        "a" => count_valid_passports::<Passport, _>,
        "b" => count_valid_passports::<StrictPassport, _>,
        _ => panic!("Bad part {}", part),
    };
    println!("{}", fun(std::io::stdin().lock()).unwrap());
}
