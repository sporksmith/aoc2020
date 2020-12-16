pub struct Game {
    prev_turn: u32,
    prev_num: u32,
    last_spoken: Vec<Option<u32>>,
}

impl Game {
    pub fn new(input: &str, max: usize) -> Game {
        let mut v = Vec::new();
        v.resize(max, None);
        let mut game = Game {
            prev_turn: 0,
            prev_num: 0,
            last_spoken: v,
        };
        for n in input.lines().next().unwrap().split(',') {
            let n = n.parse().unwrap();
            game.process_next(n);
        }
        game
    }

    pub fn process_next(&mut self, n: u32) {
        if self.prev_turn > 0 {
            self.last_spoken[self.prev_num as usize] = Some(self.prev_turn);
        }
        self.prev_num = n;
        self.prev_turn += 1;
        //println!("Turn {} spoke {}, hm: {:?}", self.prev_turn, n, self.last_spoken);
    }

    pub fn get_next(&self) -> u32 {
        if let Some(prev_turn_spoken) = self.last_spoken[self.prev_num as usize]
        {
            self.prev_turn - prev_turn_spoken
        } else {
            0
        }
    }

    pub fn run_to(&mut self, turn: u32) {
        while self.prev_turn < turn {
            self.process_next(self.get_next());
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut game = Game::new(input, 2020);
    game.run_to(2020);
    game.prev_num
}

pub fn part2(input: &str) -> u32 {
    let mut game = Game::new(input, 30000000);
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
