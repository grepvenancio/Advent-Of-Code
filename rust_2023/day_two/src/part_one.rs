fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_one(input);
    println!("Result part one: {}", output);
}

fn parse_game(game: &str) -> Option<&str> {
    let mut flag = true; 
    let before_after_id: Vec<&str> = game.split(":").collect();
    let rounds: Vec<&str> = before_after_id.last().unwrap().split(";").collect();
    for round in rounds {
        let n_cubes: Vec<&str> = round.split(",").collect();
        for play in n_cubes {
            let number_cube_color: Vec<&str> = play.split_whitespace().collect();
            let number = number_cube_color.first().unwrap().parse::<i32>().unwrap();
            let cube = number_cube_color.last().unwrap();
            match *cube {
                "green" => if number > 13 { flag = false },
                "blue" => if number > 14 { flag = false },
                "red" => if number > 12 { flag = false },
                _ => {}
            }
        }
    }
    match flag {
        true => Some(game),
        false => None
    }
}

fn solve_part_one(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            parse_game(line)
        })
        .map(|game| {
            let before_after_id = game.split(":").collect::<Vec<&str>>();
            before_after_id
                .first()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
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
