use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    // Part A
    let (mut crates, instructions) = parse_input(INPUT);
    execute_instructions(&mut crates, &instructions);
    let output = get_top_chars(&mut crates);
    println!("Part A: {output}");

    // Part B
    let (mut crates, instructions) = parse_input(INPUT);
    execute_upgraded_instructions(&mut crates, &instructions);
    let output = get_top_chars(&mut crates);
    println!("Part B: {output}");
}

#[derive(Debug, PartialEq)]
struct Instruction {
    amount: i32,
    src_index: usize,
    dest_index: usize,
}

// This enum will be used to keep track of which parsing mode is currently being used
enum ParserMode {
    Crates,
    Instructions,
}

// This trait is implemented because the amount of columns present isn't initially known.
// If you want to access the vector at index 5, you first need to create all the vectors up to index 5.
trait Expandable {
    fn populate(&mut self, new_length: usize);
}

impl<T> Expandable for Vec<Vec<T>> {
    fn populate(&mut self, new_length: usize) {
        // Start from the current length and loop to the index needed to push the sub Vec
        for _ in self.len()..=new_length {
            self.push(Vec::new());
        }
    }
}

// The parser will return two items: A map of stacks (no native Stack in std) for each column, and a list of instructions
// Each "stack" should start at the bottom. Due to the nature of the input however, each Vec will need to be reversed at the end.
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
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
                            // The inner index will be 0-indexed as the crate index is 1-indexed.
                            let index = index - 1;
                            crates.populate(index);

                            let stack = crates
                                .get_mut(index)
                                .expect("Index should already be populated");

                            stack.push(b);
                        }

                        // After each triplet, the gap character will either be a space or not exist
                        has_more = chars_iter.next().is_some();

                        if has_more {
                            index += 1;
                        }
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
                        src_index: (&captures[2]).parse::<usize>().unwrap().to_owned(),
                        dest_index: (&captures[3]).parse::<usize>().unwrap().to_owned(),
                    });
                }
            }
        }
    }

    for stack in crates.iter_mut() {
        stack.reverse();
    }

    (crates, instructions)
}

fn execute_instructions(crates: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
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
                    .get_mut(*src_index - 1)
                    .expect("Invalid instruction index or data setup");
                src_stack.pop()
            };

            // Do nothing if the stack is empty
            if let Some(token) = token {
                let dest_stack = crates
                    .get_mut(*dest_index - 1)
                    .expect("Invalid instruction index or data setup");
                dest_stack.push(token);
            }
        }
    }
}

fn get_top_chars(crates: &mut Vec<Vec<char>>) -> String {
    let mut output = String::new();

    for stack in crates.iter_mut() {
        // Columns can be empty
        if let Some(token) = stack.pop() {
            output.push(token);
        }
    }

    output
}

////////////
// Part B //
////////////

fn execute_upgraded_instructions(crates: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for Instruction {
        amount,
        src_index,
        dest_index,
    } in instructions
    {
        let mut section: Vec<char> = {
            let src_stack = crates
                .get_mut(*src_index - 1)
                .expect("Invalid instruction index or data setup");

            // Slice off the ending chunk
            // If the length is 4 (for [A, B, C, D]) and the amount you want to transfer is 3, then drain "1.."
            src_stack
                .drain(src_stack.len() - *amount as usize..)
                .collect()
        };

        let dest_stack = crates
            .get_mut(*dest_index - 1)
            .expect("Invalid instruction index or data setup");

        // Then append that ending chunk to the destination
        dest_stack.append(&mut section);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        execute_instructions, execute_upgraded_instructions, get_top_chars, parse_input,
        Instruction,
    };

    #[test]
    fn sample_parser_works() {
        let (crates, instructions) = parse_input(SAMPLE_INPUT);
        let crates2 = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

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
        let (mut crates, instructions) = parse_input(SAMPLE_INPUT);
        execute_instructions(&mut crates, &instructions);
        let crates2 = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        assert_eq!(crates, crates2);
    }

    #[test]
    fn execute_upgraded_instructions_works() {
        let (mut crates, instructions) = parse_input(SAMPLE_INPUT);
        execute_upgraded_instructions(&mut crates, &instructions);
        let crates2 = vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']];
        assert_eq!(crates, crates2);
    }

    #[test]
    fn get_top_chars_works() {
        let mut crates = vec![vec!['A', 'B'], vec!['C', 'D', 'E']];
        let output = get_top_chars(&mut crates);
        assert_eq!(output, "BE");
    }

    #[test]
    fn sample_runner_a() {
        let (mut crates, instructions) = parse_input(SAMPLE_INPUT);
        execute_instructions(&mut crates, &instructions);
        let output = get_top_chars(&mut crates);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn sample_runner_b() {
        let (mut crates, instructions) = parse_input(SAMPLE_INPUT);
        execute_upgraded_instructions(&mut crates, &instructions);
        let output = get_top_chars(&mut crates);
        assert_eq!(output, "MCD");
    }

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
