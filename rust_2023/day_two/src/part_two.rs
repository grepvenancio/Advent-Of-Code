fn main() {
    let input = include_str!("input.txt");
    let output = solve_part_two(input);
    println!("Result part two: {}", output);
}

fn parse_game(game: &str) -> i32 {
    let before_after_id: Vec<&str> = game.split(":").collect();
    let mut mim_green = 0;
    let mut mim_blue = 0;
    let mut mim_red = 0;
    let rounds: Vec<&str> = before_after_id.last().unwrap().split(";").collect();
    for round in rounds {
        let n_cubes: Vec<&str> = round.split(",").collect();
        for play in n_cubes {
            let number_cube_color: Vec<&str> = play.split_whitespace().collect();
            let number = number_cube_color.first().unwrap().parse::<i32>().unwrap();
            let cube = number_cube_color.last().unwrap();
            match *cube {
                "green" => if number > mim_green { mim_green = number},
                "blue" => if number > mim_blue { mim_blue = number },
                "red" => if number > mim_red { mim_red = number },
                _ => {}
            }
        }
    }
    let power = mim_green * mim_blue * mim_red;
    power
}

fn solve_part_two(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            parse_game(line)
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve_part_two;

    #[test]
    fn test_part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = solve_part_two(input);
        assert_eq!(output, "2286");
    }
}
