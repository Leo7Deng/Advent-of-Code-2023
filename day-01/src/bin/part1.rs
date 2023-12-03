fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut sum :u32 = 0;
    for line in input.lines() {
        let mut num1: u32 = 0;
        let mut num2: u32 = 0;
        let mut iterate :u32 = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if iterate == 0 {
                    num1 = c.to_digit(10).unwrap();
                    iterate = 1;
                }
                else if iterate >= 1 {
                    num2 = c.to_digit(10).unwrap();
                    iterate = 2;
                }
            }
            if i == line.len()-1 && iterate == 1 {
                num2 = num1;
            }
        }
        dbg!(format!("{}{}", num1, num2).to_string());
        sum += format!("{}{}", num1, num2).to_string().parse::<u32>().unwrap();
        

    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }
}