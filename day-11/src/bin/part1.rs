fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let galaxy = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let range = 0..=input.lines().nth(0).unwrap().len()-1;
    let empty_column: Vec<bool> = range
        .map(|num| galaxy.concat().contains(&(num as i32)))
        .collect();
    let empty_column = empty_column
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == &false)
        .map(|(i, _)| i as i32)
        .collect::<Vec<i32>>();

    let mut updated_galaxy = Vec::new();

    for row in galaxy.into_iter() {
        updated_galaxy.push(row.clone());

        if row.is_empty() {
            updated_galaxy.push(vec![]);
        }
    }

    dbg!(&updated_galaxy);
    for n in empty_column.iter().rev() {
        for row in &mut updated_galaxy {
            for num in row.iter_mut() {
                if *num > *n {
                    *num += 1;
                }
            }
        }
    }

    let mut coordinates = Vec::new();
    for (i, row) in updated_galaxy.iter().enumerate() {
        for x in row {
            coordinates.push((*x, i as i32));
        }
        
    }
    let mut time = 0;
    let mut distance = 0;
    for i in 0..coordinates.len() {
        for j in 1+i..coordinates.len() {
            distance += find_distance(coordinates[i], coordinates[j]);
        }
        
    }
    distance
    
    // dbg!(empty_column);
}

fn find_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....",
        );
        // ...#......
        // .......#..
        // #.........
        // ..........
        // ......#...
        // .#........
        // .........#
        // ..........
        // .......#..
        // #...#.....
        assert_eq!(result, 374);
    }
}
