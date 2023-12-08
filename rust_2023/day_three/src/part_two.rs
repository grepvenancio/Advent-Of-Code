fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_one(input);
    println!("{}", output);
}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    gear_pos: Option<Direction>
}

impl PartNumber {
    fn new(number: i32, gear_pos: Option<Direction>) -> Self {
        PartNumber { number , gear_pos }
    }
}

#[derive(Debug, PartialEq)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Self {
        Direction { x, y }
    }
}

fn solve_part_one(input: &str) -> String {
    let mut valid_numbers: Vec<PartNumber> = Vec::new();
    let input: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();
    for (x, line) in input.iter().enumerate() {
        let mut line_start = 0;
        while line_start < line.len() {
            if line[line_start].is_numeric() {
                let start_num = line_start;
                while line_start < line.len() && line[line_start].is_numeric() {
                    line_start += 1;
                }
                let end_num = line_start;
                let mut flag = false;
                let mut part_number = PartNumber::new(0, None);
                for y in start_num..end_num {
                    let dir = Direction::new(x as isize, y as isize);
                    if let Some(gear) = have_adjacent_gear(&input, dir) { 
                        flag = true; 
                        part_number.gear_pos = Some(gear);
                    }
                }
                if flag {
                    let num: String = line[start_num..end_num].iter().collect();
                    part_number.number = num.parse::<i32>().unwrap();
                    valid_numbers.push(part_number);
                }
            } else {
                line_start += 1;
            }
        }
    }
    let mut sum = 0;
    for (i, pn) in valid_numbers.iter().enumerate() {
        for inner_pn in &valid_numbers[(i+1)..] {
            if pn.gear_pos == inner_pn.gear_pos {
                let product = pn.number * inner_pn.number;
                sum += product;
            }
        }
    }
    sum.to_string()
}



fn have_adjacent_gear(input: &Vec<Vec<char>>, start_point: Direction) -> Option<Direction> {
    let directions = [
        Direction::new(-1, 0),
        Direction::new(1, 0),
        Direction::new(0, -1),
        Direction::new(0, 1),
        Direction::new(-1, -1),
        Direction::new(-1, 1),
        Direction::new(1, -1),
        Direction::new(1, 1),
    ];

    let mut adjacent_values: Vec<Direction> = Vec::new();

    for dir in &directions {
        let new_x = (start_point.x as isize + dir.x) as usize;
        let new_y = (start_point.y as isize + dir.y) as usize;

        let new_dir = Direction::new(new_x as isize, new_y as isize);

        if (new_x < input.len() && new_x > 0) && (new_y < input[new_x].len() && new_y > 0) {
            adjacent_values.push(new_dir);
        }
    }
    for dir in adjacent_values {
        if input[dir.x as usize][dir.y as usize] == '*' {
            return Some(dir)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::solve_part_one;

    #[test]
    fn test_part_one() {
        let input = "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..";
        let output = solve_part_one(input);
        assert_eq!(output, "467835");
    }
}
