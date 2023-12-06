use serde::Deserialize;

// The gist of the problem is:
// Wait 1ms to gain 1mm/ms of speed, how far can you travel in total?

// time = 7 (odd), distance = 9
// Wait 0ms to gain 0mm/ms of speed and travel 0mm over 7ms (x = 0 = 7)
// Wait 1ms to gain 1mm/ms of speed and travel 6mm over 6ms (x = 1 = 6)
// Wait 2ms to gain 2mm/ms of speed and travel 10mm over 5ms (x = 2 = 5, yes)
// Wait 3ms to gain 3mm/ms of speed and travel 12mm over 4ms (x = 3 = 4, yes)
// Wait 4ms to gain 4mm/ms of speed and travel 12mm over 3ms
// Wait 5ms to gain 5mm/ms of speed and travel 10mm over 2ms
// Wait 6ms to gain 6mm/ms of speed and travel 6mm over 1ms
// Wait 7ms to gain 7mm/ms of speed and travel 0mm over 0ms
// Wait (x) ms to gain (x) mm/ms of speed and travel (x * (time - x)) mm over (time - x) ms
// Total distance traveled should be at least the "distance" listed

// time = 4 (even)
// Wait 0ms to gain 0mm/ms of speed and travel 0mm over 4ms (x = 0 = 4)
// Wait 1ms to gain 1mm/ms of speed and travel 3mm over 3ms (x = 1 = 3)
// Wait 2ms to gain 2mm/ms of speed and travel 4mm over 2ms (x = 2)
// Wait 3ms to gain 3mm/ms of speed and travel 3mm over 1ms
// Wait 4ms to gain 4mm/ms of speed and travel 0mm over 0ms

#[derive(Debug, Deserialize)]
struct Input {
    main: Race,
    races: Vec<Race>,
}

#[derive(Debug, Deserialize)]
struct Race {
    time: i64,
    distance: i64,
}

fn main() {
    const INPUT: &str = include_str!("input/6.toml");
    let input = toml::from_str::<Input>(INPUT).expect("Invalid manual TOML");

    // Part A
    let result = calc_factor_of_races(&input);
    println!("Part A: {result}");

    // Part B
    // Apparently brute forcing is trivial
    let result = input.main.find_possible_wins();
    println!("Part B: {result}");
}

impl Race {
    fn find_possible_wins(&self) -> i64 {
        let mut sum = 0;

        for x in 0..=self.time {
            let potential_distance = x * (self.time - x);

            if potential_distance > self.distance {
                sum += 1;
            }
        }

        sum
    }
}

fn calc_factor_of_races(input: &Input) -> i64 {
    let mut factor = 1;

    for race in &input.races {
        factor *= race.find_possible_wins();
    }

    factor
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = include_str!("input-sample/6.toml");

    #[test]
    fn race_calc_works() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.find_possible_wins(), 4);

        let race = Race {
            time: 15,
            distance: 40,
        };
        assert_eq!(race.find_possible_wins(), 8);

        let race = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(race.find_possible_wins(), 9);
    }

    #[test]
    fn sample_runner_a() {
        let input = toml::from_str::<Input>(INPUT).expect("Invalid manual TOML");
        let result = calc_factor_of_races(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn sample_runner_b() {
        let input = toml::from_str::<Input>(INPUT).expect("Invalid manual TOML");
        let result = input.main.find_possible_wins();
        assert_eq!(result, 71503);
    }
}
