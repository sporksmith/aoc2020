use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
pub enum Insn {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

#[derive(Eq, PartialEq, Debug)]
pub enum RunState {
    Running,
    Done,
    Looped,
}

fn parse_line(line: &str) -> Insn {
    let mut insn_and_op = line.split(' ');
    let insn_str = insn_and_op.next().unwrap();
    let op: i32 = insn_and_op.next().unwrap().parse().unwrap();
    match insn_str {
        "nop" => Insn::Nop(op),
        "jmp" => Insn::Jmp(op),
        "acc" => Insn::Acc(op),
        _ => panic!("Bad insn"),
    }
}

pub fn parse_program<R: BufRead>(reader: R) -> Vec<Insn> {
    reader
        .lines()
        .map(|l| parse_line(l.unwrap().as_str()))
        .collect()
}

#[derive(Debug)]
pub struct Handheld<'a> {
    pc: usize,
    acc: i32,
    state: RunState,
    pcs_executed: HashSet<usize>,
    program: &'a [Insn],
}

impl<'a> Handheld<'a> {
    fn new(program: &'a [Insn]) -> Handheld {
        Handheld {
            pc: 0,
            acc: 0,
            state: RunState::Running,
            pcs_executed: HashSet::new(),
            program,
        }
    }

    fn step(&mut self) {
        if self.state != RunState::Running {
            panic!("Invalid run state");
        }
        match self.program[self.pc] {
            Insn::Nop(_) => self.pc += 1,
            Insn::Jmp(i) => self.pc = (self.pc as i32 + i) as usize,
            Insn::Acc(i) => {
                self.pc += 1;
                self.acc += i
            }
        };
        if !self.pcs_executed.insert(self.pc) {
            self.state = RunState::Looped;
        }
        if self.pc == self.program.len() {
            self.state = RunState::Done;
        }
    }
}

pub fn acc_at_loop(program: &[Insn]) -> i32 {
    let mut hh = Handheld::new(program);
    while hh.state != RunState::Looped {
        hh.step();
    }
    hh.acc
}

pub fn acc_after_fix(mut program: Vec<Insn>) -> i32 {
    // We only need to consider changing instructions that actually execute in
    // the broken version.
    let candidate_pcs = {
        let mut hh = Handheld::new(&program);
        while hh.state != RunState::Looped {
            hh.step();
        }
        hh.pcs_executed
    };

    for pc in candidate_pcs {
        // Rewrite the instruction.
        let new_insn = match &program[pc] {
            Insn::Nop(i) => Insn::Jmp(*i),
            Insn::Jmp(i) => Insn::Nop(*i),
            _ => continue,
        };
        let old_insn = program[pc];
        program[pc] = new_insn;

        // See if the program finished normally now.
        let mut hh = Handheld::new(&program);
        while hh.state == RunState::Running {
            hh.step();
        }
        if hh.state == RunState::Done {
            // Success!
            return hh.acc;
        }
        assert_eq!(hh.state, RunState::Looped);

        // Restore the old instruction and try again.
        program[pc] = old_insn;
    }
    panic!("Unfixable")
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
        assert_eq!(acc_after_fix(program), 8);
    }
}
