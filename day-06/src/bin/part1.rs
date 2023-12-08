use core::num;

use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let time_to_distance = input
        .lines()
        .map(|line| {
            Regex::new(r"\d+")
                .unwrap()
                .find_iter(line)
                .map(|num| num.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    
    let nums = time_to_distance[0]
        .iter()
        .copied()
        .zip(time_to_distance[1].iter().copied())
        .collect::<Vec<(i32, i32)>>()
        .into_iter().map(|(time, distance)| {
            let mut sum = 0;
            for speed in 1..=time {
                if speed * (time - speed) > distance {
                    sum += 1;
                }
            }
            sum
        }).collect::<Vec<i32>>();
    
    nums
        .iter()
        .fold(1, |sum, v| {
            sum * v
        })

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
        assert_eq!(result, 288);
    }
}
