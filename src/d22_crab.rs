use std::collections::{HashSet, VecDeque};

type Card = u8;
type Deck = VecDeque<Card>;

fn play_one_turn(d1: &mut Deck, d2: &mut Deck) {
    let c1 = d1.pop_front().unwrap();
    let c2 = d2.pop_front().unwrap();
    if c1 > c2 {
        d1.push_back(c1);
        d1.push_back(c2);
    } else {
        d2.push_back(c2);
        d2.push_back(c1);
    }
}

fn play_game(d1: Deck, d2: Deck) -> Deck {
    let mut d1 = d1;
    let mut d2 = d2;
    loop {
        if d1.is_empty() {
            return d2;
        }
        if d2.is_empty() {
            return d1;
        }
        play_one_turn(&mut d1, &mut d2);
    }
}

// Returns true iff player 1 wins, and returns winning deck.
fn play_rec_game(game_num: u32, d1: Deck, d2: Deck) -> (bool, Deck) {
    let mut prev_states = HashSet::new();
    let mut d1 = d1;
    let mut d2 = d2;
    //let mut round_num = 1;
    loop {
        /*
        println!("Game {} Round {}", game_num, round_num);
        println!("p1: {:?}", d1);
        println!("p2: {:?}", d2);
        round_num += 1;
        */

        let state: (Vec<Card>, Vec<Card>) =
            (d1.iter().copied().collect(), d2.iter().copied().collect());
        if !prev_states.insert(state) {
            return (true, d1);
        }

        if d1.is_empty() {
            return (false, d2);
        }
        if d2.is_empty() {
            return (true, d1);
        }

        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();

        if c1 as usize <= d1.len() && c2 as usize <= d2.len() {
            let d1new = d1.iter().take(c1 as usize).copied().collect::<Deck>();
            let d2new = d2.iter().take(c2 as usize).copied().collect::<Deck>();
            let (p1_wins, _) = play_rec_game(game_num + 1, d1new, d2new);
            if p1_wins {
                d1.push_back(c1);
                d1.push_back(c2);
            } else {
                d2.push_back(c2);
                d2.push_back(c1);
            }
        } else {
            // High card wins
            if c1 > c2 {
                d1.push_back(c1);
                d1.push_back(c2);
            } else {
                d2.push_back(c2);
                d2.push_back(c1);
            }
        }
    }
}

fn score_deck(d: &Deck) -> u64 {
    let mut multiplier = 1;
    let mut score = 0;
    for c in d.iter().rev() {
        score += *c as u64 * multiplier;
        multiplier += 1;
    }
    score
}

fn parse(input: &str) -> (Deck, Deck) {
    let mut deck_input = input.split("\n\n");
    let parse_deck = |di: &str| -> Deck {
        let mut deck = Deck::new();
        for line in di.lines().skip(1) {
            deck.push_back(line.parse().unwrap());
        }
        deck
    };
    let d1 = parse_deck(deck_input.next().unwrap());
    let d2 = parse_deck(deck_input.next().unwrap());
    (d1, d2)
}

pub fn part1(input: &str) -> u64 {
    let (d1, d2) = parse(input);
    let winner = play_game(d1, d2);
    score_deck(&winner)
}

pub fn part2(input: &str) -> u64 {
    let (d1, d2) = parse(input);
    let (_, winner) = play_rec_game(1, d1, d2);
    score_deck(&winner)
}

#[cfg(test)]
mod testing {
    use super::*;

    static INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn example() {
        let (mut d1, mut d2) = parse(INPUT);
        assert_eq!(d1, [9, 2, 6, 3, 1].iter().copied().collect::<Deck>());
        assert_eq!(d2, [5, 8, 4, 7, 10].iter().copied().collect::<Deck>());

        play_one_turn(&mut d1, &mut d2);
        assert_eq!(d1, [2, 6, 3, 1, 9, 5].iter().copied().collect::<Deck>());
        assert_eq!(d2, [8, 4, 7, 10].iter().copied().collect::<Deck>());

        assert_eq!(
            score_deck(
                &[3, 2, 10, 6, 8, 5, 9, 4, 7, 1]
                    .iter()
                    .copied()
                    .collect::<Deck>()
            ),
            306
        );
        assert_eq!(score_deck(&play_game(d1, d2)), 306);
        assert_eq!(part1(INPUT), 306);

        assert_eq!(part2(INPUT), 291);
    }
}
