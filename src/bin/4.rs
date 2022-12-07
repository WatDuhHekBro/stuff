use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    // Part A
    let data = parse_groups(INPUT);
    let amount = get_overlapping_pairs_amount(&data);
    println!("Part A: {amount}");

    // Part B
    let amount = get_intersecting_pairs_amount(&data);
    println!("Part B: {amount}");
}

fn parse_groups(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("Invalid regex");
    }
    let mut groups = Vec::new();

    for line in input.split('\n') {
        for captures in PATTERN.captures_iter(line) {
            groups.push((
                (
                    (&captures[1]).parse::<i32>().unwrap().to_owned(),
                    (&captures[2]).parse::<i32>().unwrap().to_owned(),
                ),
                (
                    (&captures[3]).parse::<i32>().unwrap().to_owned(),
                    (&captures[4]).parse::<i32>().unwrap().to_owned(),
                ),
            ));
        }
    }

    groups
}

////////////
// Part A //
////////////

fn get_overlapping_pairs_amount(data: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut count = 0;

    for (a, b) in data {
        // Check if a is overtaken by b or b is overtaken by a
        if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
            count += 1;
        }
    }

    count
}

////////////
// Part B //
////////////

fn get_intersecting_pairs_amount(data: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut count = 0;

    for (a, b) in data {
        // Check if a is overtaken by b or b is overtaken by a
        if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
            count += 1;
        }
        // Check cases where a < b, does a end before b starts?
        else if a.0 < b.0 && a.1 < b.1 && a.1 >= b.0 {
            count += 1;
        }
        // Check cases where a > b, does b end before a starts?
        else if a.0 > b.0 && a.1 > b.1 && b.1 >= a.0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{get_intersecting_pairs_amount, get_overlapping_pairs_amount, parse_groups};

    #[test]
    fn sample_parser_works() {
        let data = parse_groups(SAMPLE_INPUT);

        assert_eq!(
            data,
            vec![
                ((2, 4), (6, 8)),
                ((2, 3), (4, 5)),
                ((5, 7), (7, 9)),
                ((2, 8), (3, 7)),
                ((6, 6), (4, 6)),
                ((2, 6), (4, 8))
            ]
        );
    }

    #[test]
    fn sample_runner_a() {
        let data = parse_groups(SAMPLE_INPUT);
        let amount = get_overlapping_pairs_amount(&data);
        assert_eq!(amount, 2);
    }

    #[test]
    fn sample_runner_b() {
        let data = parse_groups(SAMPLE_INPUT);
        let amount = get_intersecting_pairs_amount(&data);
        assert_eq!(amount, 4);
    }

    const SAMPLE_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
}

const INPUT: &str = include_str!("input/4.txt");
