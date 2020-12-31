#![allow(dead_code)]

use std::error::Error;
use std::io::BufRead;

pub mod adapter;
pub mod bags;
pub mod d12_rain;
pub mod d13_bus;
pub mod d14_docking;
pub mod d15_recitation;
pub mod d16_ticket;
pub mod d17_conway;
pub mod d18_operation;
pub mod d19_messages;
pub mod d20_jigsaw;
pub mod d21_allergen;
pub mod encoding;
pub mod factors;
pub mod handheld;
pub mod passwords;
pub mod seating;
pub mod toboggan;

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
            if line.is_empty() {
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

#[cfg(test)]
#[test]
fn test_bufreadsplitonblank() {
    use std::io::Cursor;
    let input = "\
line1
line2

line3
line4";
    let reader = Cursor::new(input.as_bytes());
    let result: Vec<_> = BufReadSplitOnBlank::new(reader)
        .map(|x| x.unwrap())
        .collect();
    let expected_result: Vec<Vec<String>> = vec![
        vec!["line1".to_string(), "line2".to_string()],
        vec!["line3".to_string(), "line4".to_string()],
    ];
    assert_eq!(result, expected_result);
}
