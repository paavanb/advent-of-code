use std::fs;
use itertools::Itertools;


fn main() {
    println!("Part 1a: {}", part_1a());
    println!("Part 1b: {}", part_1b());
}

fn part_1a() -> i32 {
    let measurements = get_measurements();
    return get_increases(&measurements);
}

fn part_1b() -> i32 {
    let measurements = get_measurements();

    let windows = measurements.iter().tuple_windows::<(_, _, _)>();
    let sum = windows.map(|(a, b, c)| a + b + c);
    return get_increases(&sum.collect_vec());
}


/**
 * Grab measurements from puzzle input file.
 */
fn get_measurements() -> Vec<i32> {
    let contents = fs::read_to_string("data/sonar_sweeps.txt").expect("Something went wrong.");
    let measurements = contents.trim_end().split("\n").map(|s| s.parse::<i32>().unwrap()).collect_vec();

    return measurements;
}

/**
 * Given a vector of numbers, return the number of times the value increases over two successive
 * elements.
 */
fn get_increases(values: &[i32]) -> i32 {
    let (increases, _) = values.iter().fold((0, None), |(increases, prev), item| {
        if let Some(value) = prev {
            if value < item {
                return (increases + 1, Some(item))
            }
        }
        return (increases, Some(item))
    });

    return increases;
}
