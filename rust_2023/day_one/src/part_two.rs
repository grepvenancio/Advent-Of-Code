static NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_two(input);
    println!("Result part one: {}", output);
}

fn solve_part_two(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let parsed_vec = parse_str_to_numeric(line.trim());
            parsed_vec
        })
        .map(|inner_vec| {
            let first = inner_vec.first().unwrap();
            let last = inner_vec.last().unwrap();
            let str_number = format!("{}{}", first, last);
            str_number.parse::<i32>().unwrap()
        })
        .sum::<i32>()
        .to_string()
}

fn parse_str_to_numeric(line: &str) -> Vec<i32> {
    let mut parsed_vec: Vec<i32> = Vec::new();
    for i in 0..line.len() {
        let iter_line = &line[i..];
        for n in NUMBERS {
            if iter_line.starts_with(n) {
                parsed_vec.push(lookup_numbers(n));
            }
        }
    }
    parsed_vec
}

fn lookup_numbers(number: &str) -> i32 {
    match number {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!(),
    }
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
        assert_eq!(output, "281");
    }
}
