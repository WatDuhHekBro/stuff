fn main() {
    // Part A
    let total = run_strategy_guide(INPUT);
    println!("Part A: {total}");

    // Part B
    let total = run_reverse_strategy_guide(INPUT);
    println!("Part B: {total}");
}

enum Hands {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Results {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

////////////
// Part A //
////////////

fn execute_match(your_hand: &Hands, enemy_hand: &Hands) -> Results {
    match your_hand {
        Hands::Rock => match enemy_hand {
            Hands::Rock => Results::Draw,
            Hands::Paper => Results::Loss,
            Hands::Scissors => Results::Win,
        },
        Hands::Paper => match enemy_hand {
            Hands::Rock => Results::Win,
            Hands::Paper => Results::Draw,
            Hands::Scissors => Results::Loss,
        },
        Hands::Scissors => match enemy_hand {
            Hands::Rock => Results::Loss,
            Hands::Paper => Results::Win,
            Hands::Scissors => Results::Draw,
        },
    }
}

fn run_strategy_guide(input: &str) -> i32 {
    let mut total = 0;

    for line in input.split('\n') {
        if !line.is_empty() {
            let chars: Vec<char> = line.chars().collect();

            let opponent_strategy = match chars.get(0).expect("Invalid data") {
                'A' => Hands::Rock,
                'B' => Hands::Paper,
                'C' => Hands::Scissors,
                _ => panic!("Invalid character for opponent!"),
            };

            let your_strategy = match chars.get(2).expect("Invalid data") {
                'X' => Hands::Rock,
                'Y' => Hands::Paper,
                'Z' => Hands::Scissors,
                _ => panic!("Invalid character for player!"),
            };

            let result = execute_match(&your_strategy, &opponent_strategy);

            // Add the values of the enums
            total += your_strategy as i32 + result as i32;
        }
    }

    total
}

////////////
// Part B //
////////////

fn get_hand(result: &Results, enemy_hand: &Hands) -> Hands {
    match enemy_hand {
        Hands::Rock => match result {
            Results::Loss => Hands::Scissors,
            Results::Draw => Hands::Rock,
            Results::Win => Hands::Paper,
        },
        Hands::Paper => match result {
            Results::Loss => Hands::Rock,
            Results::Draw => Hands::Paper,
            Results::Win => Hands::Scissors,
        },
        Hands::Scissors => match result {
            Results::Loss => Hands::Paper,
            Results::Draw => Hands::Scissors,
            Results::Win => Hands::Rock,
        },
    }
}

fn run_reverse_strategy_guide(input: &str) -> i32 {
    let mut total = 0;

    for line in input.split('\n') {
        if !line.is_empty() {
            let chars: Vec<char> = line.chars().collect();

            let opponent_strategy = match chars.get(0).expect("Invalid data") {
                'A' => Hands::Rock,
                'B' => Hands::Paper,
                'C' => Hands::Scissors,
                _ => panic!("Invalid character for opponent!"),
            };

            let result_needed = match chars.get(2).expect("Invalid data") {
                'X' => Results::Loss,
                'Y' => Results::Draw,
                'Z' => Results::Win,
                _ => panic!("Invalid character for player!"),
            };

            let your_strategy = get_hand(&result_needed, &opponent_strategy);

            // Add the values of the enums
            total += your_strategy as i32 + result_needed as i32;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::{run_reverse_strategy_guide, run_strategy_guide};

    #[test]
    fn sample_runner_a() {
        let total = run_strategy_guide(SAMPLE_INPUT);
        assert_eq!(total, 15);
    }

    #[test]
    fn sample_runner_b() {
        let total = run_reverse_strategy_guide(SAMPLE_INPUT);
        assert_eq!(total, 12);
    }

    const SAMPLE_INPUT: &str = "A Y
B X
C Z
";
}

const INPUT: &str = include_str!("input/2.txt");
