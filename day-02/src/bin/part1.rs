use std::i32;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut valid: bool = true;
        for s in line.split(':').last().unwrap().split(';') {
            for c in s.split(',') {
                let mut c = c.split_whitespace();
                let num: i32 = c.next().unwrap().parse().unwrap();
                let color = c.next().unwrap();
                if !(color == "red" && num <= 12 || color == "green" && num <= 13 || color == "blue" && num <= 14) {
                    valid = false;
                }
                
            }
            
        }
        if valid {
            sum += i as i32 + 1;
            dbg!(i);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}