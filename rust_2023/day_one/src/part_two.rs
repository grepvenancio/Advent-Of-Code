fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_two(input);
    println!("Result part one: {}", output);
}


fn solve_part_two(input: &str) -> i32 {
    input
        .lines();
        todo!();
        // .map(|line| {
        //     line.matches(char::is_numeric)
        //         .map(|s| s.parse::<i32>().unwrap())
        //         .collect::<Vec<i32>>()
        // })
        // .map(|inner_vec| {
        //     let first = inner_vec.first().unwrap();
        //     let last = inner_vec.last().unwrap();
        //     let str_number = format!("{}{}", first, last);
        //     str_number.parse::<i32>().unwrap()
        // })
        // .sum()
}


#[cfg(test)]
mod tests {
    use crate::solve_part_two;

    #[test]
    fn test_part_two() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";
        let output = solve_part_two(input);
        assert_eq!(output, 281);
    }
}
