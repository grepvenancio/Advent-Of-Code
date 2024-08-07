fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_one(input);
    println!("Result part one: {}", output);
}

fn solve_part_one(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.matches(char::is_numeric)
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
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

#[cfg(test)]
mod tests {
    use crate::solve_part_one;

    #[test]
    fn test_part_one() {
        let input = "1abc2
                     pqr3stu8vwx
                     a1b2c3d4e5f
                     treb7uchet";
        let output = solve_part_one(input);
        assert_eq!(output, "142");
    }
}
