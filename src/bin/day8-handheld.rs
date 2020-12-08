use std::collections::HashSet;
use std::io::BufRead;

enum Insn {
    Nop,
    Jmp(i32),
    Acc(i32),
}

fn parse_line(line: &str) -> Insn {
    let mut insn_and_op = line.split(' ');
    let insn_str = insn_and_op.next().unwrap();
    let op: i32 = insn_and_op.next().unwrap().parse().unwrap();
    match insn_str {
        "nop" => Insn::Nop,
        "jmp" => Insn::Jmp(op),
        "acc" => Insn::Acc(op),
        _ => panic!("Bad insn"),
    }
}

fn parse_program<R: BufRead>(reader: R) -> Vec<Insn> {
    reader
        .lines()
        .map(|l| parse_line(l.unwrap().as_str()))
        .collect()
}

struct Handheld {
    pc: usize,
    acc: i32,
}

impl Handheld {
    fn new() -> Handheld {
        Handheld { pc: 0, acc: 0 }
    }

    fn execute(&mut self, insn: &Insn) {
        match insn {
            Insn::Nop => self.pc += 1,
            Insn::Jmp(i) => self.pc = (self.pc as i32 + i) as usize,
            Insn::Acc(i) => {
                self.pc += 1;
                self.acc += i
            }
        }
    }
}

fn acc_at_loop(program: &[Insn]) -> i32 {
    let mut hh = Handheld::new();
    let mut visited = HashSet::new();
    while visited.insert(hh.pc) {
        hh.execute(&program[hh.pc]);
    }
    hh.acc
}

fn main() {
    let program = parse_program(std::io::stdin().lock());
    let part = std::env::args().nth(1).expect("missing part");
    let res = match part.as_str() {
        "a" => acc_at_loop(&program),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn sample_input() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let program = parse_program(Cursor::new(input.as_bytes()));
        assert_eq!(acc_at_loop(&program), 5);
    }
}
