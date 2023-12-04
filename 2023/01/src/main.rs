use std::fs;

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> u32 {
    let lines = get_calibration_lines();
    return lines
        .iter()
        .map(|line| get_simple_calibration_value(&line))
        .sum();
}

fn part_2() -> u32 {
    let lines = get_calibration_lines();
    return lines
        .iter()
        .map(|line| get_complex_calibration_value(&line))
        .sum();
}

fn get_simple_calibration_value(line: &str) -> u32 {
    let (first, last) = line
        .chars()
        .fold((None, None), |(maybe_first, maybe_last), c| {
            match c.to_digit(10) {
                None => (maybe_first, maybe_last),
                Some(digit) => {
                    match maybe_first {
                        // We haven't found the first digit yet, initialize
                        None => (Some(digit), Some(digit)),
                        // Already found the first digit, just update last
                        Some(_) => (maybe_first, Some(digit)),
                    }
                }
            }
        });
    return first.unwrap() * 10 + last.unwrap();
}

fn get_complex_calibration_value(line: &str) -> u32 {
    // Replace all spelled out numbers with numbers, then pass to part 1 impl
    let substituted_string = line
        .replace("one", "o1e") // This substitution allows words to overlap by one character, e.g., "twone". Definitely a hack!
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    return get_simple_calibration_value(&substituted_string);
}

fn get_calibration_lines() -> Vec<String> {
    let contents = fs::read_to_string("data/calibration_doc.txt").expect("Something went wrong.");
    let lines = contents.trim_end().lines().map(String::from).collect();

    return lines;
}
