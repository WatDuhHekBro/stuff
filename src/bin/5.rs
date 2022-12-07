use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Part A
    let (mut crates, instructions, length) = parse_input(INPUT);
    execute_instructions(&mut crates, &instructions);
    let output = get_top_chars(&mut crates, length);
    println!("Part A: {output}");

    // Part B
    println!("Part B: ");
}

#[derive(Debug, PartialEq)]
struct Instruction {
    amount: i32,
    src_index: i32,
    dest_index: i32,
}

// This enum will be used to keep track of which parsing mode is currently being used
enum ParserMode {
    Crates,
    Instructions,
}

// The parser will return two items: A map of stacks (no native Stack in std) for each column, and a list of instructions
// If a Vec was used to hold the stacks, you'd have to implement extra logic to create all the Vecs before (if the first crate found started at index 5 for example)
// Each "stack" should start at the bottom. Due to the nature of the input however, each Vec will need to be reversed at the end.
fn parse_input(input: &str) -> (HashMap<i32, Vec<char>>, Vec<Instruction>, i32) {
    let mut crates: HashMap<i32, Vec<char>> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut length = 0;
    let mut mode = ParserMode::Crates;

    for line in input.split('\n') {
        if !line.is_empty() {
            match mode {
                ParserMode::Crates => {
                    let mut chars_iter = line.chars();
                    let mut has_more = true;
                    let mut index = 1;

                    while has_more {
                        // Read 3 chars at a time: "[c]", "   ", or " 5 "
                        let a = chars_iter.next().expect("Invalid data");
                        let b = chars_iter.next().expect("Invalid data");
                        let c = chars_iter.next().expect("Invalid data");

                        // If this triplet is an index, then move onto the next state
                        if a == ' ' && b != ' ' && c == ' ' {
                            mode = ParserMode::Instructions;
                            break; // None of the indexes are actually needed, it's just an indicator to move onto the next mode
                        }
                        // Parse the character and push it onto the target stack if there's a crate
                        // Otherwise, if no crate exists at this triplet, move on
                        else if a == '[' && c == ']' {
                            // HashMap::insert will overwrite, so check if something already exists there
                            if !crates.contains_key(&index) {
                                crates.insert(index, Vec::new());
                            }

                            let stack = crates
                                .get_mut(&index)
                                .expect("Index should already be populated");

                            stack.push(b);
                        }

                        // After each triplet, the gap character will either be a space or not exist
                        has_more = chars_iter.next().is_some();

                        if has_more {
                            index += 1;
                        }
                    }

                    // Take the highest index reached of each row
                    if length < index {
                        length = index;
                    }
                }
                ParserMode::Instructions => {
                    lazy_static! {
                        static ref PATTERN: Regex =
                            Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Invalid regex");
                    }

                    let captures = PATTERN.captures(line).expect("Invalid data");

                    instructions.push(Instruction {
                        amount: (&captures[1]).parse::<i32>().unwrap().to_owned(),
                        src_index: (&captures[2]).parse::<i32>().unwrap().to_owned(),
                        dest_index: (&captures[3]).parse::<i32>().unwrap().to_owned(),
                    });
                }
            }
        }
    }

    for (_, stack) in crates.iter_mut() {
        stack.reverse();
    }

    (crates, instructions, length)
}

fn execute_instructions(crates: &mut HashMap<i32, Vec<char>>, instructions: &Vec<Instruction>) {
    for Instruction {
        amount,
        src_index,
        dest_index,
    } in instructions
    {
        for _ in 0..*amount {
            // Due to the fact that only one mutable reference is allowed to exist at a time, this ugliness must ensue
            let token = {
                let src_stack = crates
                    .get_mut(src_index)
                    .expect("Invalid instruction index or data setup");
                src_stack.pop()
            };

            // Do nothing if the stack is empty
            if let Some(token) = token {
                let dest_stack = crates
                    .get_mut(dest_index)
                    .expect("Invalid instruction index or data setup");
                dest_stack.push(token);
            }
        }
    }
}

fn get_top_chars(crates: &mut HashMap<i32, Vec<char>>, length: i32) -> String {
    let mut output = String::new();

    for index in 1..=length {
        let stack = crates.get_mut(&index).expect("HashMap not setup properly");
        stack.reverse();

        if let Some(token) = stack.pop() {
            output.push(token);
        }
    }

    output

    /*let keys = crates.keys();
    // Because HashMaps don't ever give out a proper order of their keys, this requires the janky solution of sorting the keys then using them again.
    let mut ordered_keys = Vec::new();

    for key in keys {
        ordered_keys.push(key);
    }

    ordered_keys.sort_unstable();

    for key in ordered_keys {
        crates.get_mut(key).unwrap();
    }*/
}

////////////
// Part A //
////////////

////////////
// Part B //
////////////

#[cfg(test)]
mod tests {
    use crate::{execute_instructions, get_top_chars, parse_input, Instruction};
    use std::collections::HashMap;

    #[test]
    fn sample_parser_works() {
        let (crates, instructions, length) = parse_input(SAMPLE_INPUT);

        let mut crates2 = HashMap::new();
        crates2.insert(1, vec!['Z', 'N']);
        crates2.insert(2, vec!['M', 'C', 'D']);
        crates2.insert(3, vec!['P']);

        assert_eq!(length, 3);

        assert_eq!(crates, crates2);
        assert_eq!(
            instructions,
            vec![
                Instruction {
                    amount: 1,
                    src_index: 2,
                    dest_index: 1,
                },
                Instruction {
                    amount: 3,
                    src_index: 1,
                    dest_index: 3,
                },
                Instruction {
                    amount: 2,
                    src_index: 2,
                    dest_index: 1,
                },
                Instruction {
                    amount: 1,
                    src_index: 1,
                    dest_index: 2,
                }
            ]
        );
    }

    #[test]
    fn execute_instructions_works() {
        let (mut crates, instructions, _) = parse_input(SAMPLE_INPUT);
        execute_instructions(&mut crates, &instructions);

        let mut crates2 = HashMap::new();
        crates2.insert(1, vec!['C']);
        crates2.insert(2, vec!['M']);
        crates2.insert(3, vec!['P', 'D', 'N', 'Z']);

        assert_eq!(crates, crates2);
    }

    // Currently failing test for arcane reasons, need to investigate later
    #[test]
    fn get_top_chars_works() {
        let mut crates = HashMap::new();
        crates.insert(1, vec!['A', 'B']);
        crates.insert(2, vec!['X', 'Y', 'Z']);

        let asdf = crates.get_mut(&2).unwrap();
        asdf.reverse();

        // Reads ['A', 'B'] then ['X', 'Y'] only in a for loop, really weird, need to look into this
        for index in 1..=2 {
            let stack = crates.get_mut(&index).expect("HashMap not setup properly");
            let a = stack.len();
            let b = a + 1;
            println!("{b}");
            stack.reverse();

            if let Some(token) = stack.pop() {
                print!("{token}");
            }
        }

        let output = get_top_chars(&mut crates, 3);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn sample_runner_a() {
        let (mut crates, instructions, length) = parse_input(SAMPLE_INPUT);
        execute_instructions(&mut crates, &instructions);
        let output = get_top_chars(&mut crates, length);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn sample_runner_b() {}

    const SAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
}

const INPUT: &str = include_str!("input/5.txt");
