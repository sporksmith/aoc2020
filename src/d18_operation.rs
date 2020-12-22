#![allow(dead_code)]

mod v1 {
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    enum Token {
        LParen,
        RParen,
        Plus,
        Times,
        Val(u64),
    }

    fn tokenize(s: &str) -> Vec<Token> {
        let mut res = Vec::new();
        let mut s = s;
        while !s.is_empty() {
            s = s.trim_start();
            match s.chars().next().unwrap() {
                '(' => {
                    res.push(Token::LParen);
                    s = &s[1..];
                }
                ')' => {
                    res.push(Token::RParen);
                    s = &s[1..];
                }
                '+' => {
                    res.push(Token::Plus);
                    s = &s[1..];
                }
                '*' => {
                    res.push(Token::Times);
                    s = &s[1..];
                }
                _ => {
                    let end = match s.find(|c: char| !c.is_digit(10)) {
                        Some(i) => i,
                        None => s.len(),
                    };
                    res.push(Token::Val(s[..end].parse().unwrap()));
                    s = &s[end..];
                }
            };
        }

        res
    }
    #[cfg(test)]
    #[test]
    fn test_tokenize() {
        use Token::*;
        assert_eq!(
            tokenize("1 + 2 * (32 + 4)"),
            vec![
                Val(1),
                Plus,
                Val(2),
                Times,
                LParen,
                Val(32),
                Plus,
                Val(4),
                RParen
            ]
        );
    }

    fn p1_priority(t: &Token) -> u8 {
        use Token::*;
        match t {
            LParen => 0,
            Plus | Times => 1,
            RParen => 2,
            _ => panic!("Unexpected token {:?}", t),
        }
    }

    fn conventional_priority(t: &Token) -> u8 {
        use Token::*;
        match t {
            LParen => 0,
            Plus => 1,
            Times => 2,
            RParen => 3,
            _ => panic!("Unexpected token {:?}", t),
        }
    }

    fn p2_priority(t: &Token) -> u8 {
        use Token::*;
        match t {
            LParen => 0,
            Times => 1,
            Plus => 2,
            RParen => 3,
            _ => panic!("Unexpected token {:?}", t),
        }
    }

    fn to_rpn<F: Fn(&Token) -> u8>(
        priority: F,
        tokens: &[Token],
    ) -> Vec<Token> {
        let mut res = Vec::<Token>::new();
        let mut ops_stack = Vec::<Token>::new();
        for token in tokens {
            use Token::*;
            match token {
                RParen => loop {
                    use Token::*;
                    match ops_stack.pop().expect("Unbalanced parens") {
                        Plus => res.push(Plus),
                        Times => res.push(Times),
                        LParen => break,
                        x => panic!("Unexpected {:?}", x),
                    }
                },
                LParen => ops_stack.push(*token),
                Plus | Times => {
                    while !ops_stack.is_empty()
                        && priority(&ops_stack[ops_stack.len() - 1])
                            >= priority(&token)
                    {
                        res.push(ops_stack.pop().unwrap());
                    }
                    ops_stack.push(*token)
                }
                Val(_) => res.push(*token),
            }
        }
        while let Some(popped) = ops_stack.pop() {
            res.push(popped);
        }
        res
    }
    #[cfg(test)]
    #[test]
    fn test_torpn() {
        assert_eq!(
            to_rpn(conventional_priority, &tokenize("1 + 2 * 3")),
            tokenize("1 2 3 * +")
        );
        assert_eq!(
            to_rpn(p1_priority, &tokenize("1 + 2 * 3")),
            tokenize("1 2 + 3 *")
        );
        assert_eq!(
            to_rpn(p2_priority, &tokenize("1 + 2 * 3")),
            tokenize("1 2 + 3 *")
        );

        assert_eq!(
            to_rpn(conventional_priority, &tokenize("1 * 2 + 3")),
            tokenize("1 2 * 3 +")
        );
        assert_eq!(
            to_rpn(p1_priority, &tokenize("1 * 2 + 3")),
            tokenize("1 2 * 3 +")
        );
        assert_eq!(
            to_rpn(p2_priority, &tokenize("1 * 2 + 3")),
            tokenize("1 2 3 + *")
        );

        assert_eq!(
            to_rpn(p1_priority, &tokenize("(1 + 2)")),
            tokenize("1 2 +")
        );

        assert_eq!(
            to_rpn(p1_priority, &tokenize("1 + (2 * 3)")),
            tokenize("1 2 3 * +")
        );
    }

    fn eval_rpn(tokens: &[Token]) -> u64 {
        let mut stack = Vec::<u64>::new();
        for token in tokens {
            use Token::*;
            match token {
                Plus => {
                    let lhs = stack.pop().unwrap();
                    let rhs = stack.pop().unwrap();
                    stack.push(lhs + rhs);
                }
                Times => {
                    let lhs = stack.pop().unwrap();
                    let rhs = stack.pop().unwrap();
                    stack.push(lhs * rhs);
                }
                Val(x) => stack.push(*x),
                _ => panic!("Unexpected token {:?}", token),
            }
        }
        assert_eq!(stack.len(), 1);
        stack[0]
    }
    #[cfg(test)]
    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(&tokenize("1 2 +")), 3);
        assert_eq!(eval_rpn(&tokenize("1 2 *")), 2);
        assert_eq!(eval_rpn(&tokenize("1 2 3 * +")), 7);
        assert_eq!(eval_rpn(&tokenize("1 2 + 3 *")), 9);
    }

    pub fn p1_eval(s: &str) -> u64 {
        eval_rpn(&to_rpn(p1_priority, &tokenize(s)))
    }
    #[cfg(test)]
    #[test]
    fn test_p1_eval() {
        assert_eq!(p1_eval("1 + 2 + 3"), 6);
        assert_eq!(p1_eval("1 + 2 * 3"), 9);
        assert_eq!(p1_eval("(1 + 2) * 3"), 9);
        assert_eq!(p1_eval("1 + (2 * 3)"), 7);

        assert_eq!(p1_eval("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(p1_eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(p1_eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(p1_eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(p1_eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            p1_eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    pub fn p2_eval(s: &str) -> u64 {
        eval_rpn(&to_rpn(p2_priority, &tokenize(s)))
    }
    #[cfg(test)]
    #[test]
    fn test_p2_eval() {
        assert_eq!(p2_eval("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(p2_eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(p2_eval("2 * 3 + (4 * 5)"), 46);
        assert_eq!(p2_eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            p2_eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            p2_eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}

mod v0 {
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

    pub fn take_expr(s: &str) -> (u64, &str) {
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
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(v1::p1_eval).sum()
}
#[cfg(test)]
#[test]
fn test_part1() {
    let input = "\
1 + 2
3 * 4";
    assert_eq!(part1(input), 3 + 12);
}

pub fn part2(input: &str) -> u64 {
    input.lines().map(v1::p2_eval).sum()
}
#[cfg(test)]
#[test]
fn test_part2() {
    let input = "\
1 + 2
3 * 4";
    assert_eq!(part1(input), 3 + 12);
}
