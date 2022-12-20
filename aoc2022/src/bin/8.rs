fn main() {
    // 5 --> 2 inclusive
    for a in (2..=5).rev() {
        println!("{a}");
    }
    // Part A
    println!("Part A: ");

    // Part B
    println!("Part B: ");
}

// It's possible to make this a bitflag of 4 bits, but eh
struct Visibility {
    up: bool,
    left: bool,
    right: bool,
    down: bool,
}

struct Forest {
    // 2D grid represented by a 1D vector
    // Each tree can be 0-9
    trees: Vec<u8>,
    size: u8,
}

impl Forest {
    fn new(size: u8) -> Forest {
        Forest {
            trees: Vec::with_capacity((size * size) as usize),
            size,
        }
    }

    fn push(&mut self, element: u8) {
        self.trees.push(element);
    }

    // Get a 0-indexed (x, y) pair from the index
    // size = 5, 0 = (0, 0), 1 = (1, 0), 4 = (4, 0), 5 = (0, 1)
    fn get_coordinates(&self, index: u8) -> (u8, u8) {
        (index % self.size, index / self.size)
    }

    // Utility function to modularize checking each direction for visibility
    fn get_visibility_in_direction() -> bool {
        let mut is_visible = true;

        for y in y..self.size {
            let index = y * self.size + x;
            let index = index as usize;

            let tree = *self
                .trees
                .get(index)
                .expect("Forest::get_visibility @ down - index out of bounds");

            // If the selected tree is ever >= the current tree, exit immediately.
            if tree >= current {
                is_visible = false;
            }
        }

        // The tree will only remain visible if it never gets blocked during the check.
        is_visible
    }

    fn get_visibility(&self, x: u8, y: u8) -> Visibility {
        if x == 0 || x == self.size - 1 || y == 0 || y == self.size - 1 {
            Visibility {
                up: true,
                left: true,
                right: true,
                down: true,
            }
        } else {
            let current = {
                let index = y * self.size + x;

                *self
                    .trees
                    .get(index as usize)
                    .expect("Forest::get_visibility @ self - index out of bounds")
            };

            let up = {
                //
                false
            };

            let left = {
                //
                false
            };

            let right = {
                //
                false
            };

            let down = {
                let mut is_visible = true;

                for y in y..self.size {
                    let index = y * self.size + x;
                    let index = index as usize;

                    let tree = *self
                        .trees
                        .get(index)
                        .expect("Forest::get_visibility @ down - index out of bounds");

                    // If the selected tree is ever >= the current tree, exit immediately.
                    if tree >= current {
                        is_visible = false;
                    }
                }

                // The tree will only remain visible if it never gets blocked during the check.
                is_visible
            };

            Visibility {
                up,
                left,
                right,
                down,
            }
        }
    }
}

fn parse_forest(input: &str) -> Forest {
    let mut iter = input.lines().peekable();
    let first_line = iter.peek().expect("The provided input is empty.");
    let mut forest = Forest::new((*first_line).len() as u8);

    for line in iter {
        // Loop through each character and parse it into a u8
        for char in line.chars() {
            let tree = char
                .to_digit(10)
                .expect("Invalid base 10 digit found in input");
            forest.push(tree as u8);
        }
    }

    forest
}

////////////
// Part A //
////////////

////////////
// Part B //
////////////

#[cfg(test)]
mod tests {
    use crate::parse_forest;

    #[test]
    fn sample_parser_works() {
        let forest = parse_forest(SAMPLE_INPUT);

        assert_eq!(
            forest.trees,
            vec![3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0]
        );
    }

    #[test]
    fn visibility_works() {
        let forest = parse_forest(SAMPLE_INPUT);

        // Row #1
        assert!(forest.is_visible(0, 0));
        assert!(forest.is_visible(1, 0));
        assert!(forest.is_visible(2, 0));
        assert!(forest.is_visible(3, 0));
        assert!(forest.is_visible(4, 0));

        // Row #2
        assert!(forest.is_visible(0, 1));
        assert!(!forest.is_visible(1, 1));
        assert!(!forest.is_visible(2, 1));
        assert!(!forest.is_visible(3, 1));
        assert!(forest.is_visible(4, 1));

        // Row #3
        assert!(forest.is_visible(0, 2));
        assert!(!forest.is_visible(1, 2));
        assert!(!forest.is_visible(2, 2));
        assert!(!forest.is_visible(3, 2));
        assert!(forest.is_visible(4, 2));

        // Row #4
        assert!(forest.is_visible(0, 3));
        assert!(!forest.is_visible(1, 3));
        assert!(!forest.is_visible(2, 3));
        assert!(!forest.is_visible(3, 3));
        assert!(forest.is_visible(4, 3));

        // Row #5
        assert!(forest.is_visible(0, 4));
        assert!(forest.is_visible(1, 4));
        assert!(forest.is_visible(2, 4));
        assert!(forest.is_visible(3, 4));
        assert!(forest.is_visible(4, 4));
    }

    #[test]
    fn sample_runner_a() {}

    #[test]
    fn sample_runner_b() {}

    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390
";
}

const INPUT: &str = include_str!("input/8.txt");
