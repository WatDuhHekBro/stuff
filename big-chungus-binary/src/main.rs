use std::env;

fn main() {
    // First gather args (except first element)
    let args = &env::args().collect::<Vec<String>>()[1..].to_vec();

    // Then join everything together if args include tokens in quotes (e.g. ./program "big chungus")
    // ...

    // Compute and print
    let out = compute(args);
    println!(
        "Input Text: {}\nBinary: {out:b}\nHex: {out:x}",
        args.join(" ")
    );
}

fn compute(input: &Vec<String>) -> u64 {
    // Then adding bits assuming big endian, where big = 0 and chungus = 1
    let mut output = 0;
    let length = input.len();

    for index in 0..length {
        let power = length - index - 1;

        if input[index] == "chungus" {
            output += 1 << power;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // chungus
    // 1
    // 1
    #[test]
    fn one() {
        assert_eq!(compute(&vec!["chungus".to_string()]), 1);
    }

    // big big chungus big chungus big chungus
    // 0010101
    // 21
    #[test]
    fn two() {
        assert_eq!(
            compute(&vec![
                "big".to_string(),
                "big".to_string(),
                "chungus".to_string(),
                "big".to_string(),
                "chungus".to_string(),
                "big".to_string(),
                "chungus".to_string()
            ]),
            21
        );
    }
}
