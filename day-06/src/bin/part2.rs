use core::num;

use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let time_to_distance = input
        .lines()
        .map(|line| {
            Regex::new(r"\d+")
                .unwrap()
                .find_iter(line)
                .map(|num| num.as_str().parse::<String>().unwrap())
                .collect::<Vec<String>>()
                .join("")
        })
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
 
        let mut sum = 0;
        for speed in 1..=time_to_distance[0] {
            if speed * (time_to_distance[0] - speed) > time_to_distance[1] {
                sum += 1;
            }
        }
        sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
        Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }
}
