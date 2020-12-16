use std::collections::HashMap;

pub struct Game {
    prev_turn: u64,
    prev_num: u64,
    last_spoken: HashMap<u64, u64>,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let mut game = Game {
            prev_turn: 0,
            prev_num: 0,
            last_spoken: HashMap::new(),
        };
        for n in input.lines().next().unwrap().split(',') {
            let n: u64 = n.parse().unwrap();
            game.process_next(n);
        }
        game
    }

    pub fn process_next(&mut self, n: u64) {
        if self.prev_turn > 0 {
            self.last_spoken.insert(self.prev_num, self.prev_turn);
        }
        self.prev_num = n;
        self.prev_turn += 1;
        //println!("Turn {} spoke {}, hm: {:?}", self.prev_turn, n, self.last_spoken);
    }

    pub fn get_next(&self) -> u64 {
        if let Some(prev_turn_spoken) = self.last_spoken.get(&self.prev_num) {
            self.prev_turn - prev_turn_spoken
        } else {
            0
        }
    }

    pub fn run_to(&mut self, turn: u64) {
        while self.prev_turn < turn {
            self.process_next(self.get_next());
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut game = Game::new(input);
    game.run_to(2020);
    game.prev_num
}

pub fn part2(input: &str) -> u64 {
    let mut game = Game::new(input);
    game.run_to(30000000);
    game.prev_num
}

#[cfg(test)]
#[test]
fn test_example() {
    assert_eq!(part1("0,3,6\n"), 436);
    // Passes but sloooow
    //assert_eq!(part2("0,3,6\n"), 175594);
}
