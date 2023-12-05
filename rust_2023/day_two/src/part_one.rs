fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_one(input);
    println!("Result part one: {}", output);
}

fn parse_game(game: &str) -> Option<&str> {
    let mut flag = true;
    for round in game.split(":").last().unwrap().split(";") {
        for play in round.split(",").map(|play| play.trim()) {
            let whitespace_after_num = play.find(" ").unwrap();
            let number = play[..whitespace_after_num].parse::<i32>().unwrap();
            let cube = &play[(whitespace_after_num + 1)..];
            match cube {
                "green" => {
                    if number > 13 {
                        flag = false
                    }
                }
                "blue" => {
                    if number > 14 {
                        flag = false
                    }
                }
                "red" => {
                    if number > 12 {
                        flag = false
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    match flag {
        true => Some(game),
        false => None,
    }
}

fn get_id(game: &str) -> i32 {
    let game_id_gap = game.find(" ").unwrap();
    let colon = game.find(":").unwrap();
    game[(game_id_gap + 1)..colon].parse::<i32>().unwrap()
}

fn solve_part_one(input: &str) -> String {
    input
        .lines()
        .filter(|line| parse_game(line).is_some())
        .map(|game| get_id(game.trim()))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve_part_one;

    #[test]
    fn test_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = solve_part_one(input);
        assert_eq!(output, "8");
    }
}
