fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input.lines().enumerate().flat_map(|(line_num, line)| {
        line.chars().map(move |c| {
            if c.is_numeric {

            }
            (line_num as i32, 
        })
    }).collect::<Vec<(i32, char)>>().len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}