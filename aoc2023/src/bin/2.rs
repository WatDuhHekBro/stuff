const INPUT: &str = include_str!("input/2");

fn main() {
    // Part A
    let games = parse_file_into_game_list(INPUT);
    let sum = get_valid_games_sum(&games, 12, 13, 14);
    println!("Part A: {sum}");

    // Part B
    let sum = get_minimum_cube_sum(&games);
    println!("Part B: {sum}");
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_file_into_game_list(input: &str) -> Vec<Game> {
    let mut game_list = Vec::new();

    // For each game
    // Step #1 = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
    for line in input.lines() {
        // Step #2 = ["Game 1"], [" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
        let game: Vec<&str> = line.split(':').collect();

        if game.len() != 2 {
            panic!("Expected game to have one colon: \"{line}\"");
        }

        let sets = game[1];
        let game = game[0];

        // Parse "Game 1" into ID
        if !game.starts_with("Game ") {
            panic!("Game does not start with \"Game \": \"{line}\"");
        }

        let game = &game[5..];
        let game = game
            .parse::<i32>()
            .expect(&format!("Expected proper game ID for: \"{line}\""));

        // For each set
        // Step #3 = ["Game 1"], [" 3 blue, 4 red"], [" 1 red, 2 green, 6 blue"], [" 2 green"]
        let sets: Vec<&str> = sets.split(';').collect();
        let mut set_list: Vec<Set> = Vec::new();

        // For each collection of cubes
        // Step #4 = ["Game 1"], [" 3 blue", " 4 red"], [" 1 red", " 2 green", " 6 blue"], [" 2 green"]
        for set in sets {
            let cubes: Vec<&str> = set.split(',').collect();
            let mut current_set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };

            for cube in cubes {
                // Trim whitespace
                let cube = cube.trim();

                // Split by space (should only be one space)
                let tmp: Vec<&str> = cube.split(' ').collect();

                if tmp.len() != 2 {
                    panic!(
                        "Expected each collection of cube sections to have one space: \"{line}\""
                    );
                }

                let amount = tmp[0];
                let amount = amount.parse::<i32>().expect("Expected proper integer.");
                let color = tmp[1];

                match color {
                    "red" => {
                        current_set.red = amount;
                    }
                    "green" => {
                        current_set.green = amount;
                    }
                    "blue" => {
                        current_set.blue = amount;
                    }
                    unknown => panic!("Unknown color \"{unknown}\" in line: \"{line}\""),
                }
            }

            set_list.push(current_set);
        }

        game_list.push(Game {
            id: game,
            sets: set_list,
        });
    }

    game_list
}

// Check if the game's colors exceed the constraint's colors at any point
fn is_game_possible_with_constraints(game: &Game, red: i32, green: i32, blue: i32) -> bool {
    for set in &game.sets {
        if set.red > red || set.green > green || set.blue > blue {
            return false;
        }
    }

    true
}

// Add up all game IDs that pass the constraint checks
fn get_valid_games_sum(games: &Vec<Game>, red: i32, green: i32, blue: i32) -> i32 {
    let mut sum = 0;

    for game in games {
        let is_possible = is_game_possible_with_constraints(&game, red, green, blue);

        if is_possible {
            sum += game.id;
        }
    }

    sum
}

// Find out minimum cubes needed for each game (max of each game), then multiply "red * green * blue", then add it up
fn get_minimum_cube_sum(games: &Vec<Game>) -> i32 {
    let mut sum = 0;

    for game in games {
        let mut max_red = i32::MIN;
        let mut max_green = i32::MIN;
        let mut max_blue = i32::MIN;

        for set in &game.sets {
            if set.red > max_red {
                max_red = set.red;
            }

            if set.green > max_green {
                max_green = set.green;
            }

            if set.blue > max_blue {
                max_blue = set.blue;
            }
        }

        let power = max_red * max_green * max_blue;
        sum += power;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = include_str!("input-sample/2");

    #[test]
    fn parser_works() {
        let game_list = parse_file_into_game_list(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"#,
        );

        assert_eq!(
            game_list,
            vec![
                Game {
                    id: 1,
                    sets: vec![
                        Set {
                            blue: 3,
                            red: 4,
                            green: 0
                        },
                        Set {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Set {
                            green: 2,
                            red: 0,
                            blue: 0
                        }
                    ]
                },
                Game {
                    id: 2,
                    sets: vec![
                        Set {
                            blue: 1,
                            green: 2,
                            red: 0
                        },
                        Set {
                            green: 3,
                            blue: 4,
                            red: 1
                        },
                        Set {
                            green: 1,
                            blue: 1,
                            red: 0
                        }
                    ]
                }
            ]
        );
    }

    #[test]
    fn constraints_checker_works() {
        assert!(!is_game_possible_with_constraints(
            &Game {
                id: 333,
                sets: vec![
                    Set {
                        blue: 8,
                        red: 20,
                        green: 6
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Set {
                        green: 2,
                        red: 0,
                        blue: 0
                    }
                ]
            },
            12,
            13,
            14
        ))
    }

    #[test]
    fn sample_runner_a() {
        let games = parse_file_into_game_list(SAMPLE_INPUT);
        let sum = get_valid_games_sum(&games, 12, 13, 14);
        assert_eq!(sum, 8);
    }

    #[test]
    fn sample_runner_b() {
        let games = parse_file_into_game_list(SAMPLE_INPUT);
        let sum = get_minimum_cube_sum(&games);
        assert_eq!(sum, 2286);
    }
}
