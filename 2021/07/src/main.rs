use std::fs;

const DATA_FILE: &str = "data/crab_submarines.txt";

fn main() {
    println!("Part one: {}", get_minimum_fuel(DATA_FILE, &linear_fuel_cost));
    println!("Part two: {}", get_minimum_fuel(DATA_FILE, &arithmetic_fuel_cost));
}

fn get_minimum_fuel(filename: &str, cost_fn: &dyn Fn(&Vec<u16>, u16) -> u32) -> u32 {
    let positions = get_positions(filename);
    let &min = positions.iter().min().expect("Expected at least one position.");
    let &max = positions.iter().max().expect("Expected at least one position.");

    let mut minimum_fuel = u32::MAX;
    for pos in min..max {
        minimum_fuel = minimum_fuel.min(cost_fn(&positions, pos));
    }

    return minimum_fuel;
}

fn linear_fuel_cost(positions: &Vec<u16>, target: u16) -> u32 {
    positions.iter().map(|&p| (p as i32 - target as i32).abs() as u32).sum()
}

fn arithmetic_fuel_cost(positions: &Vec<u16>, target: u16) -> u32 {
    positions.iter().map(|&p| {
        let abs_difference = (p as i32 - target as i32).abs();
        // arithmetic_sum = n_terms * (first_term + last_term) / 2
        let arithmetic_sum = abs_difference * (1 + abs_difference) / 2;
        return arithmetic_sum as u32;
    }).sum()
}

fn get_positions(filename: &str) -> Vec<u16> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split(",")
        .map(|s| s.parse::<u16>().expect("Expected u16."))
        .collect()
}
