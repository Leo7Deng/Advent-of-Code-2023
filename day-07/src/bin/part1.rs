use std::{cmp::Ordering, iter::zip};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_whitespace().collect_tuple().unwrap();
            let mut repeat: HashMap<char, usize> = HashMap::new();
            for c in x.chars() {
                *repeat.entry(c).or_insert(0) += 1
            }
            let mut repeat: Vec<usize> = repeat.values().cloned().collect();
            repeat.sort();
            let strength;
            if repeat[0] == 5 {
                strength = 7;
            } else if repeat[0] == 1 && repeat[1] == 4 {
                strength = 6;
            } else if repeat[0] == 2 && repeat[1] == 3 {
                strength = 5;
            } else if repeat[0] == 1 && repeat[1] == 1 && repeat[2] == 3 {
                strength = 4;
            } else if repeat[0] == 1 && repeat[1] == 2 && repeat[2] == 2 {
                strength = 3;
            } else if repeat[0] == 1 && repeat[1] == 1 && repeat[2] == 1 && repeat[3] == 2 {
                strength = 2;
            } else {
                strength = 1;
            }

            (x.to_string(), y.parse::<usize>().unwrap(), strength)
        })
        .collect::<Vec<(String, usize, usize)>>();
    hands.sort_by(|(hand, _, strength), (hand2, _, strength2)| {
        match strength.cmp(&strength2) {
            Ordering::Equal => zip(hand.chars(), hand2.chars()).find_map(|(card, card2)| {
                match (card.is_alphabetic(), card2.is_alphabetic()) {
                    (true, true) => {
                        let card = match card {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 11,
                            'T' => 10,
                            _ => todo!()
                        };
                        let card2 = match card2 {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 11,
                            'T' => 10,
                            _ => todo!()
                        };
                        match card.cmp(&card2) {
                            Ordering::Equal => None,
                            ord => Some(ord),
                        }
                    },
                    (true, false) => Some(Ordering::Greater),
                    (false, true) => Some(Ordering::Less),
                    (false, false) => match card.to_digit(10).cmp(&card2.to_digit(10)) {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    }
                    
                }
            }).unwrap(),
            ord => ord,
        }
    });
    hands.iter().enumerate().fold(0, |acc, (i,(_, bet, _))| acc + ((i+1) * bet))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}
