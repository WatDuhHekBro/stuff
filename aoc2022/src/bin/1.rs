fn main() {
    // Part A
    let mut data = get_processed_input(INPUT);
    data.sort_unstable();
    data.reverse();
    println!("Part A: {}", data.get(0).unwrap());

    // Part B
    let sum = data.get(0).unwrap() + data.get(1).unwrap() + data.get(2).unwrap();
    println!("Part B: {sum}");
}

fn get_processed_input(input: &str) -> Vec<i32> {
    let mut list = Vec::new(); // elf # = index, calories = output
    let mut sum = 0;
    let mut has_queued = false;

    for line in input.split('\n') {
        // If the line is an empty string, then process a new elf.
        if line.is_empty() {
            list.push(sum);
            sum = 0;
            has_queued = false;
        } else {
            let parsed: i32 = line
                .parse()
                .expect("The input data should be a list of numbers.");
            sum += parsed;
            has_queued = true;
        }
    }

    // If there's still a value that hasn't been pushed, push it.
    if has_queued {
        list.push(sum);
    }

    list
}

#[cfg(test)]
mod tests {
    use crate::get_processed_input;

    #[test]
    fn sample_parser_works() {
        let data = get_processed_input(SAMPLE_INPUT);
        assert_eq!(data, Vec::from([6000, 4000, 11000, 24000, 10000]));
    }

    #[test]
    fn sample_parser_newline() {
        let one = get_processed_input("1\n2\n3");
        let two = get_processed_input("1\n2\n3\n");
        assert_eq!(one, two);
    }

    #[test]
    fn sample_runner_a() {
        let mut data = get_processed_input(SAMPLE_INPUT);
        data.sort_unstable();
        data.reverse();
        assert_eq!(data.get(0).unwrap().to_owned(), 24000);
    }

    #[test]
    fn sample_runner_b() {
        let mut data = get_processed_input(SAMPLE_INPUT);
        data.sort_unstable();
        data.reverse();
        let sum = data.get(0).unwrap() + data.get(1).unwrap() + data.get(2).unwrap();
        assert_eq!(sum, 45000);
    }

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
}

const INPUT: &str = include_str!("input/1.txt");
