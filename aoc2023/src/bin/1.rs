const INPUT: &str = include_str!("input/1");

fn main() {
    // Part A
    let sum = get_calibration_sum(INPUT);
    println!("Part A: {sum}");

    // Part B
    let sum = get_calibration_sum_complex(INPUT);
    println!("Part B: {sum}");
}

// <file> --> "12345"
fn get_calibration_sum(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += get_calibration_value_of_line(line);
    }

    sum
}

// "a1b2c3d4e5f" --> "15" (first & last digits, always 2-digit number)
fn get_calibration_value_of_line(line: &str) -> u32 {
    let mut value = 0;
    let mut last_digit: Option<u32> = None;

    for c in line.chars() {
        let digit = c.to_digit(10);

        // Found a digit?
        if let Some(digit) = digit {
            // Is this the first digit?
            if last_digit.is_none() {
                value = digit * 10;
            }

            last_digit = Some(digit);
        }
    }

    // Add the other digit together
    value += last_digit.expect("Expected at least one digit in this line.");

    value
}

// <file> --> "12345"
fn get_calibration_sum_complex(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += get_calibration_value_of_line_complex(line);
    }

    sum
}

// "zoneight234" --> "14" (first & last digits, always 2-digit number, spelled-out literals now)
fn get_calibration_value_of_line_complex(line: &str) -> u32 {
    let mut value = 0;
    let mut last_digit: Option<u32> = None;

    for (index, c) in line.chars().enumerate() {
        let text_digit = get_digit_of_spelled_out_digit(&line[index..]);
        let digit = c.to_digit(10);

        // Found a digit?
        if let Some(digit) = digit {
            // Is this the first digit?
            if last_digit.is_none() {
                value = digit * 10;
            }

            last_digit = Some(digit);
        }
        // Found a text digit?
        else if let Some(digit) = text_digit {
            // Is this the first digit?
            if last_digit.is_none() {
                value = digit * 10;
            }

            last_digit = Some(digit);
        }
    }

    // Add the other digit together
    value += last_digit.expect("Expected at least one digit in this line.");

    value
}

// "eightasdfzxcv" --> "8" (use starts_with because of variable lengths)
// yandev moment
fn get_digit_of_spelled_out_digit(text: &str) -> Option<u32> {
    if text.starts_with("zero") {
        Some(0)
    } else if text.starts_with("one") {
        Some(1)
    } else if text.starts_with("two") {
        Some(2)
    } else if text.starts_with("three") {
        Some(3)
    } else if text.starts_with("four") {
        Some(4)
    } else if text.starts_with("five") {
        Some(5)
    } else if text.starts_with("six") {
        Some(6)
    } else if text.starts_with("seven") {
        Some(7)
    } else if text.starts_with("eight") {
        Some(8)
    } else if text.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT_A: &str = include_str!("input-sample/1a");
    const SAMPLE_INPUT_B: &str = include_str!("input-sample/1b");

    #[test]
    fn sample_runner_a() {
        let sum = get_calibration_sum(SAMPLE_INPUT_A);
        assert_eq!(sum, 142);
    }

    #[test]
    fn sample_runner_b() {
        let sum = get_calibration_sum_complex(SAMPLE_INPUT_B);
        assert_eq!(sum, 281);
    }
}
