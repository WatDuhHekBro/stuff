use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

///////////////////
// Brainstorming //
///////////////////
// HashMap to link transformers (e.g. "seed" to "soil"), contains objects as values
// Transformer struct to parse input, transform via parsed rules, with a "next" property (if exists) to link to next transformer

lazy_static! {
    // "seeds: 79 14 55 13" --> ("79 14 55 13")
    static ref PATTERN_SEEDS: Regex = Regex::new(r"seeds: (.+)").unwrap();
    // "seed-to-soil map:" --> ("seed", "soil")
    static ref PATTERN_RULE: Regex = Regex::new(r"(\w+?)-to-(\w+?) map:").unwrap();
}

const INPUT: &str = include_str!("input/5");

fn main() {
    // Part A
    let (seeds, transformers) = parse_file_into_seeds_and_transformer_map_pair(INPUT);
    let result = find_lowest_location_number(&seeds, &transformers);
    println!("Part A: {result}");

    // Part B
    // It's pretty obvious that you're not meant to brute force this
    //let seeds = convert_raw_seeds_into_seed_ranges(&seeds);
    //let result = find_lowest_location_number(&seeds, &transformers);
    //println!("Part B: {result}");
}

#[derive(Debug, Eq, PartialEq)]
struct Transformer {
    rules: Vec<Rule>,
    next: String, // would be Option<String> if the header structure was different
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    offset: i64, // i64 because my input data includes values around 4 billion
    range: Range<i64>,
}

impl Transformer {
    fn new(next: String) -> Transformer {
        Transformer {
            rules: Vec::new(),
            next,
        }
    }

    fn add_rule(&mut self, line: &str) {
        // Interpret the text for each rule
        let parsed_numbers: Vec<i64> = line
            .split(' ')
            .map(|x| x.parse().expect(&format!("Invalid i64 {x}")))
            .collect();

        if parsed_numbers.len() != 3 {
            panic!("Parsed numbers array \"{parsed_numbers:?}\" must be length 3!");
        }

        let destination_range_start = parsed_numbers[0];
        let source_range_start = parsed_numbers[1];
        let range_length = parsed_numbers[2];

        // Then calculate useful info like additive offsets and range object
        // src + offset = dst === dst - src = offset
        // 98 + (-48) = 50 === 50 - 98 = -48
        let offset = destination_range_start - source_range_start;
        // e.g. 98..100 covers 98 and 99
        let range = source_range_start..(source_range_start + range_length);

        self.rules.push(Rule { offset, range });
    }

    fn transform(&self, number: i64) -> i64 {
        // Search the list of rules in index and apply if exists (otherwise just pass the number itself without transforming)
        for rule in &self.rules {
            if rule.range.contains(&number) {
                return number + rule.offset;
            }
        }

        number
    }
}

fn parse_file_into_seeds_and_transformer_map_pair(
    file: &str,
) -> (Vec<i64>, HashMap<String, Transformer>) {
    let mut lines_iter = file.lines();

    // Initial seeds (assume starts at the top of file)
    let matches = PATTERN_SEEDS
        .captures(lines_iter.next().expect("Empty file?!"))
        .expect("No matches?!");
    let seeds = &matches[1];
    let seeds = seeds.split(' ').map(|x| x.parse().unwrap()).collect();
    assert_eq!(
        lines_iter.next().unwrap(),
        "",
        "Next line should be empty (after initial seeds)"
    );

    // Then setup the transformers
    let mut transformers = HashMap::new();
    let mut from = String::new();
    let mut current_transformer: Option<Transformer> = None;

    for line in lines_iter {
        // Attempt to parse the header for the current rule
        let matches = PATTERN_RULE.captures(line);

        // If successful, update current header
        if let Some(matches) = matches {
            from = String::from(&matches[1]);
            let to = &matches[2];
            current_transformer = Some(Transformer::new(to.into()));
        }
        // If there is some text, it's a rule
        else if !line.is_empty() {
            if let Some(ref mut current_transformer) = current_transformer {
                current_transformer.add_rule(line);
            } else {
                panic!("No current transformer set?!");
            }
        }
        // Otherwise if it's empty, it's the end of the ruleset
        else {
            transformers.insert(
                from.clone().into(),
                current_transformer.expect("No current transformer set when adding the result?!"),
            );
            current_transformer = None;
        }
    }

    // Make sure to also add the last transformer in a file due if there isn't a trailing line
    if let Some(current_transformer) = current_transformer {
        transformers.insert(from.clone().into(), current_transformer);
    }

    (seeds, transformers)
}

fn find_location_number_of_seed(seed: i64, transformers: &HashMap<String, Transformer>) -> i64 {
    // Start with the transformer "seed"
    let mut current_transformer = transformers
        .get("seed")
        .expect("There should be an entry in the HashMap for \"seed\".");
    let mut result = seed;

    loop {
        // Transform the value
        result = current_transformer.transform(result);

        // Stop searching if ending on "location"
        if current_transformer.next == "location" {
            break;
        }

        // Otherwise, get the next transformer
        current_transformer = transformers.get(&current_transformer.next).expect("This should only break if \"location\" isn't actually the final value for some reason.");
    }
    // Quick sanity check
    println!(
        "[Quick Sanity Check] Finding Location Number: Currently working with seed of \"{seed}\", result is \"{result}\"."
    );

    result
}

fn find_lowest_location_number(
    seeds: &Vec<i64>,
    transformers: &HashMap<String, Transformer>,
) -> i64 {
    seeds
        .iter()
        .map(|seed| find_location_number_of_seed(*seed, &transformers))
        .min()
        .expect("Empty iterator.")
}

// First generates a range, then loops through it to create a new seed list
fn convert_raw_seeds_into_seed_ranges(seeds: &Vec<i64>) -> Vec<i64> {
    let mut new_seeds = Vec::new();
    let mut iter = seeds.iter();

    loop {
        let start = iter.next();

        // End loop if end of seeds list
        if let Some(start) = start {
            let length = iter
                .next()
                .expect("Seeds list cannot have an odd number of elements.");
            // Quick sanity check
            println!("[Quick Sanity Check] Seed Processing: Currently working with start value of \"{start}\" and length \"{length}\".");

            // Loop through the generated range and add the new seeds
            for seed in *start..(start + length) {
                new_seeds.push(seed);
            }
        } else {
            break;
        }
    }

    new_seeds
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = include_str!("input-sample/5");

    #[test]
    fn transformer_works() {
        let mut transformer = Transformer::new("doesn't matter lmao".into());
        transformer.add_rule("50 98 2");
        transformer.add_rule("52 50 48");

        // Ripped from base examples on problem page
        assert_eq!(transformer.transform(98), 50);
        assert_eq!(transformer.transform(99), 51);
        assert_eq!(transformer.transform(53), 55);
        assert_eq!(transformer.transform(10), 10);
    }

    #[test]
    fn parser_works() {
        let mut transformer = Transformer::new("soil".into());
        transformer.add_rule("50 98 2");
        transformer.add_rule("52 50 48");

        let mut map: HashMap<String, Transformer> = HashMap::new();
        map.insert("seed".into(), transformer);

        assert_eq!(
            parse_file_into_seeds_and_transformer_map_pair(
                r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48"
            ),
            (vec![79, 14, 55, 13], map)
        );
    }

    #[test]
    fn find_location_works() {
        let (_, transformers) = parse_file_into_seeds_and_transformer_map_pair(SAMPLE_INPUT);
        assert_eq!(find_location_number_of_seed(79, &transformers), 82);
        assert_eq!(find_location_number_of_seed(14, &transformers), 43);
        assert_eq!(find_location_number_of_seed(55, &transformers), 86);
        assert_eq!(find_location_number_of_seed(13, &transformers), 35);
    }

    #[test]
    fn convert_seeds_works() {
        let seeds = convert_raw_seeds_into_seed_ranges(&vec![79, 2, 55, 3]);
        assert_eq!(seeds, vec![79, 80, 55, 56, 57])
    }

    #[test]
    fn sample_runner_a() {
        let (seeds, transformers) = parse_file_into_seeds_and_transformer_map_pair(SAMPLE_INPUT);
        assert_eq!(find_lowest_location_number(&seeds, &transformers), 35);
    }

    #[test]
    fn sample_runner_b() {
        let (seeds, transformers) = parse_file_into_seeds_and_transformer_map_pair(SAMPLE_INPUT);
        let seeds = convert_raw_seeds_into_seed_ranges(&seeds);
        assert_eq!(find_lowest_location_number(&seeds, &transformers), 46);
    }
}
