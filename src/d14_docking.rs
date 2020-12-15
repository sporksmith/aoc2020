use std::collections::HashMap;

/// ```
/// use aoc2020::d14_docking::*;
/// assert_eq!(parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), (0b1000000, 0b10));
/// ```
pub fn parse_mask(mask: &str) -> (u64, u64) {
    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;
    for c in mask.chars() {
        set_mask <<= 1;
        clear_mask <<= 1;
        match c {
            '0' => clear_mask |= 1,
            '1' => set_mask |= 1,
            _ => (),
        };
    }
    (set_mask, clear_mask)
}

pub fn part1(input: &str) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut set_mask = 0u64;
    let mut clear_mask = 0u64;
    for line in input.lines() {
        if line.starts_with("mask") {
            let (new_set_mask, new_clear_mask) =
                parse_mask(&line["mask = ".len()..]);
            set_mask = new_set_mask;
            clear_mask = new_clear_mask;
        } else if line.starts_with("mem") {
            let mut lhs_rhs = line.split(" = ");
            let lhs = lhs_rhs.next().unwrap();
            let rhs = lhs_rhs.next().unwrap();
            let addr: u64 = lhs["mem[".len()..lhs.len() - 1].parse().unwrap();
            let val: u64 = rhs.parse().unwrap();
            mem.insert(addr, (val | set_mask) & !clear_mask);
        }
    }
    mem.values().sum()
}

/// ```
/// use aoc2020::d14_docking::*;
/// assert_eq!(parse_mask_v2("000000000000000000000000000000X1001X"), (0b10010, vec![0b100000, 0b1]));
/// ```
pub fn parse_mask_v2(mask: &str) -> (u64, Vec<u64>) {
    let mut set_mask = 0u64;
    let mut floating = Vec::<u64>::new();
    let mut bit = mask.len();
    for c in mask.chars() {
        set_mask <<= 1;
        bit -= 1;
        match c {
            '1' => set_mask |= 1,
            'X' => floating.push(1 << bit),
            _ => (),
        };
    }
    (set_mask, floating)
}

fn floating_write(
    mem: &mut HashMap<u64, u64>,
    addr: u64,
    val: u64,
    floating: &[u64],
) {
    if floating.is_empty() {
        mem.insert(addr, val);
        return;
    }
    let mask = floating[0];
    let floating = &floating[1..];
    // Write with floating bit set.
    floating_write(mem, addr | mask, val, floating);
    // Write with floating bit unset.
    floating_write(mem, addr & !mask, val, floating);
}

pub fn part2(input: &str) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut set_mask = 0u64;
    let mut floating = Vec::<u64>::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            let (new_set_mask, new_floating) =
                parse_mask_v2(&line["mask = ".len()..]);
            set_mask = new_set_mask;
            floating = new_floating;
        } else if line.starts_with("mem") {
            let mut lhs_rhs = line.split(" = ");
            let lhs = lhs_rhs.next().unwrap();
            let rhs = lhs_rhs.next().unwrap();
            let addr: u64 = lhs["mem[".len()..lhs.len() - 1].parse().unwrap();
            let val: u64 = rhs.parse().unwrap();
            floating_write(&mut mem, addr | set_mask, val, &floating);
        }
    }
    mem.values().sum()
}

#[cfg(test)]
#[test]
fn test_example() {
    let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(part1(input), 165);

    let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    assert_eq!(part2(input), 208);
}
