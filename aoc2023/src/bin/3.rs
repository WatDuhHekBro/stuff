use std::collections::HashMap;

use grid::*;

const INPUT: &str = include_str!("input/3");

fn main() {
    // Part A
    let grid = parse_file_into_entity_grid(INPUT);
    let map = find_adjacent_numbers(&grid);
    let sum = sum_adjacent_numbers_from_map(&map);
    println!("Part A: {sum}");

    // Part B
    let sum = find_gear_ratios(&grid);
    println!("Part B: {sum}");
}

// A point on the grid with something (number or part)
#[derive(Debug, Eq, PartialEq)]
enum Entity {
    Number(EntityNumber),
    EnginePart(char),
}

// A number on the grid with an ID to avoid adding duplicate values
#[derive(Debug, Eq, PartialEq)]
struct EntityNumber {
    value: i32,
    id: u16,
}

fn parse_file_into_entity_grid(file: &str) -> Grid<Option<Entity>> {
    let mut grid = Grid::new(0, 0); // Will automatically panic if column # is different
    let mut id = 0; // Global ID

    // line = "467..114.."
    for line in file.lines() {
        // Iterate through each character with index
        let mut row: Vec<Option<Entity>> = Vec::new();
        let mut current_clipboard: Option<String> = None; // If currently grabbing a number

        for c in line.chars() {
            match c {
                '.' => {
                    // First push clipboard of digits if tracking
                    if let Some(ref clip) = current_clipboard {
                        let number = clip.parse::<i32>().unwrap();

                        for _ in 0..clip.len() {
                            row.push(Some(Entity::Number(EntityNumber { value: number, id })));
                        }

                        // Then make sure to clear the clipboard
                        current_clipboard = None;
                        // Also make sure to increment the ID
                        id += 1;
                    }

                    // Then push the current value
                    row.push(None);
                }
                // Start tracking digits
                '0'..='9' => {
                    if let Some(ref mut clip) = current_clipboard {
                        clip.push(c);
                    } else {
                        current_clipboard = Some(String::from(c));
                    }
                }
                _ => {
                    // First push clipboard of digits if tracking
                    if let Some(ref clip) = current_clipboard {
                        let number = clip.parse::<i32>().unwrap();

                        for _ in 0..clip.len() {
                            row.push(Some(Entity::Number(EntityNumber { value: number, id })));
                        }

                        // Then make sure to clear the clipboard
                        current_clipboard = None;
                        // Also make sure to increment the ID
                        id += 1;
                    }

                    // Then push the current value
                    row.push(Some(Entity::EnginePart(c)));
                }
            }
        }

        // Push clipboard of digits if the digits were right at the end
        if let Some(ref clip) = current_clipboard {
            let number = clip.parse::<i32>().unwrap();

            for _ in 0..clip.len() {
                row.push(Some(Entity::Number(EntityNumber { value: number, id })));
            }

            // Also make sure to increment the ID
            id += 1;
        }

        grid.push_row(row);
    }

    grid
}

const ADJACENT_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    //(0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// Loop through the grid and check each tile adjacent to an engine part
// Returns a HashMap of numbers for easy duplication checks
fn find_adjacent_numbers(grid: &Grid<Option<Entity>>) -> HashMap<u16, i32> {
    let mut map = HashMap::new();

    for (r, row) in grid.iter_rows().enumerate() {
        for (c, col) in row.enumerate() {
            // Check if it's an engine part
            if let Some(Entity::EnginePart(_)) = col {
                // Then check adjacent tiles
                for (r_offset, c_offset) in ADJACENT_OFFSETS {
                    // Don't bother if the index is 0 and the offset is -1
                    if !((r == 0 && r_offset == -1) || (c == 0 && c_offset == -1)) {
                        // Dirty casting
                        let r = ((r as i32) + r_offset) as usize;
                        let c = ((c as i32) + c_offset) as usize;
                        let value = grid.get(r, c);

                        // 1st Option checks if value is in bounds
                        // Then checks if value is a number
                        if let Some(Some(Entity::Number(number))) = value {
                            map.insert(number.id, number.value);
                        }
                    }
                }
            }
        }
    }

    map
}

fn sum_adjacent_numbers_from_map(map: &HashMap<u16, i32>) -> i32 {
    let mut sum = 0;

    for (_id, number) in map.iter() {
        sum += number;
    }

    sum
}

// Loop through the grid and check each tile adjacent to an engine part
// Returns a HashMap of numbers for easy duplication checks
fn find_gear_ratios(grid: &Grid<Option<Entity>>) -> i32 {
    let mut sum = 0;

    for (r, row) in grid.iter_rows().enumerate() {
        for (c, col) in row.enumerate() {
            // Check if it's a gear
            if let Some(Entity::EnginePart('*')) = col {
                // Create a temporary HashMap to track adjacent numbers for that gear
                let mut map = HashMap::new();

                // Then check adjacent tiles
                for (r_offset, c_offset) in ADJACENT_OFFSETS {
                    // Don't bother if the index is 0 and the offset is -1
                    if !((r == 0 && r_offset == -1) || (c == 0 && c_offset == -1)) {
                        // Dirty casting
                        let r = ((r as i32) + r_offset) as usize;
                        let c = ((c as i32) + c_offset) as usize;
                        let value = grid.get(r, c);

                        // 1st Option checks if value is in bounds
                        // Then checks if value is a number
                        if let Some(Some(Entity::Number(number))) = value {
                            map.insert(number.id, number.value);
                        }
                    }
                }

                // Finally, check if there are two entries
                if map.len() == 2 {
                    // Then calculate the gear ratio and add it to the running total if so
                    let mut entries = map.iter();
                    let (_, a) = entries.next().unwrap();
                    let (_, b) = entries.next().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = include_str!("input-sample/3");

    #[test]
    fn parser_works() {
        let grid = parse_file_into_entity_grid(
            r#"467..114..
..5*......"#,
        );

        assert_eq!(
            grid,
            grid![[
                Some(Entity::Number(EntityNumber { value: 467, id: 0 })),
                Some(Entity::Number(EntityNumber { value: 467, id: 0 })),
                Some(Entity::Number(EntityNumber { value: 467, id: 0 })),
                None,
                None,
                Some(Entity::Number(EntityNumber { value: 114, id: 1 })),
                Some(Entity::Number(EntityNumber { value: 114, id: 1 })),
                Some(Entity::Number(EntityNumber { value: 114, id: 1 })),
                None,
                None
            ][
                None, None, Some(Entity::Number(EntityNumber { value: 5, id: 2 })), Some(Entity::EnginePart('*')), None, None, None, None, None, None
            ]]
        );
    }

    #[test]
    fn check_adjacent_numbers() {
        let grid = parse_file_into_entity_grid(
            r#"467..114..
...*......"#,
        );
        let map = find_adjacent_numbers(&grid);

        let mut known_map = HashMap::new();
        known_map.insert(0, 467);

        assert_eq!(map, known_map);
    }

    #[test]
    fn sample_runner_a() {
        let grid = parse_file_into_entity_grid(SAMPLE_INPUT);
        let map = find_adjacent_numbers(&grid);
        let sum = sum_adjacent_numbers_from_map(&map);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn sample_runner_b() {
        let grid = parse_file_into_entity_grid(SAMPLE_INPUT);
        let sum = find_gear_ratios(&grid);
        assert_eq!(sum, 467835);
    }
}
