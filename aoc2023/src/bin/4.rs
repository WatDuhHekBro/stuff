use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

lazy_static! {
    // "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53" --> (1) (41 ... 17) (83 ... 53)
    static ref PATTERN_LINE: Regex = Regex::new(r"Card\s+(\d+): (.+?) \| (.+)").unwrap();
    static ref PATTERN_VAR_WHITESPACE: Regex = Regex::new(r"\s{1,2}").unwrap();
}

const INPUT: &str = include_str!("input/4");

fn main() {
    // Part A
    let cards = parse_file_into_cards(INPUT);
    let sum = get_sum_points_of_card_list(&cards);
    println!("Part A: {sum}");

    // Part B
    let amount = get_total_amount_of_cards(&cards);
    println!("Part B: {amount}");
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    id: u16,
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

fn parse_file_into_cards(file: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in file.lines() {
        let matches = PATTERN_LINE.captures(line).unwrap();
        let id = &matches[1];
        let winning_numbers = &matches[2].trim();
        let your_numbers = &matches[3].trim();

        // Parse accordingly
        let id: u16 = id.parse().unwrap();
        let winning_numbers: Vec<i32> = PATTERN_VAR_WHITESPACE
            .split(&winning_numbers)
            .map(|x| x.parse().unwrap())
            .collect();
        let your_numbers: Vec<i32> = PATTERN_VAR_WHITESPACE
            .split(&your_numbers)
            .map(|x| x.parse().unwrap())
            .collect();

        // Bring it all together
        let card = Card {
            id,
            winning_numbers,
            your_numbers,
        };
        cards.push(card);
    }

    cards
}

fn get_points_of_card(card: &Card) -> i32 {
    let base: i32 = 2;
    let mut power = 0;

    // For each winning number, check if it exists and increment the power
    for number in &card.winning_numbers {
        if card.your_numbers.contains(number) {
            power += 1;
        }
    }

    // 0 is the base case, 2^0 = 1 is for 1 winning number
    if power == 0 {
        0
    } else {
        power -= 1;
        base.pow(power)
    }
}

fn get_sum_points_of_card_list(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;

    for card in cards {
        sum += get_points_of_card(card);
    }

    sum
}

// If n has 4 copies, get one copy each of n+1 to n+4.
// Sample: c1 = 4, c2 = 2, c3 = 2, c4 = 1, c5 = 0, c6 = 0
fn get_copies_of_card(card: &Card) -> i32 {
    let mut copies = 0;

    // For each winning number, check if it exists and increment the amount of copies
    for number in &card.winning_numbers {
        if card.your_numbers.contains(number) {
            copies += 1;
        }
    }

    copies
}

// Sample Copies (including original):
// c1 = 1, c2 = 1, c3 = 1, c4 = 1, c5 = 1, c6 = 1 (start)
// c1 = 1, c2 = 2, c3 = 2, c4 = 2, c5 = 2, c6 = 1 (c1, 1 copy for 2-5)
// c1 = 1, c2 = 2, c3 = 4, c4 = 4, c5 = 2, c6 = 1 (c2, 2 copies for 3-4)
// c1 = 1, c2 = 2, c3 = 4, c4 = 8, c5 = 6, c6 = 1 (c3, 4 copies for 4-5)
// c1 = 1, c2 = 2, c3 = 4, c4 = 8, c5 = 14, c6 = 1 (c4, 8 copies for 5)
// sum = 1 + 2 + 4 + 8 + 14 + 1 = 30
fn get_total_amount_of_cards(cards: &Vec<Card>) -> i32 {
    let length = cards.len();

    // Initialize an array with necessary capacity (but still at symbolic length = 0)
    let mut copies_array: Vec<i32> = Vec::with_capacity(length);
    // Then resize that array to meet the specified length, filling the entries with 1 as the base to work with
    copies_array.resize(length, 1);

    for (index, card) in cards.iter().enumerate() {
        let copies = copies_array[index];
        let amount_cards_to_affect = get_copies_of_card(card);

        // Create a sub loop for the amount of cards to affect, then add copies for each sub-index
        let sub_index_start = index + 1;
        let sub_index_end = index + (amount_cards_to_affect as usize);
        // Make sure that the ending index (inclusive) doesn't go out of bounds
        let sub_index_end = cmp::min(sub_index_end, length - 1);

        // Don't attempt to index into out of bounds territory
        if sub_index_start < length {
            for sub_index in sub_index_start..=sub_index_end {
                copies_array[sub_index] += copies;
            }
        }
    }

    copies_array.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = include_str!("input-sample/4");

    #[test]
    fn parser_works() {
        let cards = parse_file_into_cards("Card 420: 41 48 83 | 83 86  6  9");

        assert_eq!(
            cards,
            vec![Card {
                id: 420,
                winning_numbers: vec![41, 48, 83],
                your_numbers: vec![83, 86, 6, 9]
            }]
        );
    }

    #[test]
    fn points_system_works() {
        let cards = parse_file_into_cards("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let points = get_points_of_card(&cards[0]);
        assert_eq!(points, 8);
    }

    #[test]
    fn sample_runner_a() {
        let cards = parse_file_into_cards(SAMPLE_INPUT);
        let sum = get_sum_points_of_card_list(&cards);
        assert_eq!(sum, 13);
    }

    #[test]
    fn sample_runner_b() {
        let cards = parse_file_into_cards(SAMPLE_INPUT);
        let amount = get_total_amount_of_cards(&cards);
        assert_eq!(amount, 30);
    }
}
