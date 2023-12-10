use std::{cmp::Ordering, iter::zip};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let cards = input.lines().map(|line|{
        let (win, nums) = line.split(':').nth(1).unwrap().split('|').collect_tuple().unwrap();
        let win = win
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<String>>();
        let mut numbers = HashMap::new();
        let individual_number: Vec<&str> = nums.trim().split_whitespace().collect();
        for c in individual_number {
            numbers.insert(c.to_string(), 1);
        }
        (win, numbers)
    }).collect::<Vec<(Vec<String>, HashMap<String, i32>)>>();
    let num_win = cards.iter().map(|(win, num)| {
        let sum = win.iter().fold(0, |acc, x| {
            if num.contains_key(x) {
                if acc == 0 {
                    acc + 1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        });
        sum
    }).collect::<Vec<i32>>();
    num_win.iter().fold(0, |acc, x| acc + x)    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
