#[derive(Eq, PartialEq, Debug)]
enum Operator {
    Plus,
    Times,
}

fn take_num(s: &str) -> (u64, &str) {
    let end = match s.find(|c: char| !c.is_digit(10)) {
        Some(i) => i,
        None => s.len(),
    };
    let val = s[..end].parse().unwrap();
    (val, &s[end..])
}
#[cfg(test)]
#[test]
fn test_take_num() {
    assert_eq!(take_num("28"), (28, ""));
    assert_eq!(take_num("28 + 3"), (28, " + 3"));
}

fn take_operator(s: &str) -> (Operator, &str) {
    let op = if s.starts_with(" + ") {
        Operator::Plus
    } else if s.starts_with(" * ") {
        Operator::Times
    } else {
        panic!("Unrecognized operator '{}'", s);
    };
    (op, &s[3..])
}
#[cfg(test)]
#[test]
fn test_take_operator() {
    assert_eq!(take_operator(" + 3"), (Operator::Plus, "3"));
}

fn take_operand(s: &str) -> (u64, &str) {
    if let Some(s) = s.strip_prefix('(') {
        let (val, s) = take_expr(s);
        assert_eq!(s.chars().next(), Some(')'));
        let s = &s[1..];
        //let s = s.trim_start();
        (val, s)
    } else {
        take_num(s)
    }
}
#[cfg(test)]
#[test]
fn test_take_operand() {
    assert_eq!(take_operand("(1 + 2) + 3"), (3, " + 3"));
    assert_eq!(take_operand("1 + 2 + 3"), (1, " + 2 + 3"));
}

fn take_expr(s: &str) -> (u64, &str) {
    let (val, s) = take_operand(s);
    let mut val = val;
    let mut s = s;
    while !s.is_empty() && !s.starts_with(')') {
        let (op, new_s) = take_operator(s);
        s = new_s;
        let (rhs, new_s) = take_operand(s);
        s = new_s;
        val = match op {
            Operator::Plus => val + rhs,
            Operator::Times => val * rhs,
        }
    }
    (val, s)
}
#[cfg(test)]
#[test]
fn test_take_expr() {
    assert_eq!(take_expr("1 + 2 + 3"), (6, ""));
    assert_eq!(take_expr("1 + 2 * 3"), (9, ""));
    assert_eq!(take_expr("(1 + 2) * 3"), (9, ""));
    assert_eq!(take_expr("1 + (2 * 3)"), (7, ""));

    assert_eq!(take_expr("1 + 2 * 3 + 4 * 5 + 6"), (71, ""));
    assert_eq!(take_expr("1 + (2 * 3) + (4 * (5 + 6))"), (51, ""));
    assert_eq!(take_expr("2 * 3 + (4 * 5)"), (26, ""));
    assert_eq!(take_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), (437, ""));
    assert_eq!(
        take_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        (12240, "")
    );
    assert_eq!(
        take_expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        (13632, "")
    );
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (val, s) = take_expr(l);
            assert_eq!(s, "");
            val
        })
        .sum()
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = "\
1 + 2
3 * 4";
    assert_eq!(part1(input), 3 + 12);
}
