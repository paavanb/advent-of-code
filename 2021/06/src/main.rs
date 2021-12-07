use std::fs;
use std::collections::HashMap;

const DATA_FILE: &str = "data/lanternfish.txt";

// Map from counter value to the number of fish with that counter value
type FishState = HashMap<u8, u64>;


fn main() {
    let fish_state = get_fish_state(DATA_FILE);
    println!("Part one: {}", simulate_fish(&fish_state, 80));
    println!("Part two: {}", simulate_fish(&fish_state, 256));
}

/// Simulate n days given initial fish state
fn simulate_fish(fish_state: &FishState, days: u16) -> u64 {
    let mut new_fish_state: FishState = fish_state.clone();
    for _ in 0..days {
        new_fish_state = update_state(&new_fish_state);
    }
    return new_fish_state.values().into_iter().sum();
}

/// Given the current fish state, advance one day and return the new state.
fn update_state(fish_state: &FishState) -> FishState {
    let mut new_state: FishState = HashMap::new();
    for &counter in fish_state.keys() {
        let &num_fish = fish_state.get(&counter).unwrap_or(&0);
        if counter == 0 {
            // Reset parent fish counter to 6
            add_fish(&mut new_state, 6, num_fish);

            // Create new child fish with counter 8
            add_fish(&mut new_state, 8, num_fish);
        } else {
            add_fish(&mut new_state, counter - 1, num_fish);
        }
    }

    return new_state;
}

fn add_fish(fish_state: &mut FishState, counter: u8, amount: u64) {
    fish_state.insert(
        counter,
        amount + if fish_state.contains_key(&counter) { fish_state[&counter] } else { 0 }
    );
}

fn get_fish_state(filename: &str) -> FishState {
    let data = get_data(filename);
    let mut fish_state: FishState = HashMap::new();

    for value in data {
        if fish_state.contains_key(&value) {
            let prev = fish_state[&value];
            fish_state.insert(value, prev + 1);
        } else {
            fish_state.insert(value, 1);
        }
    }

    return fish_state;
}

fn get_data(filename: &str) -> Vec<u8> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split(",")
        .map(|s| s.parse::<u8>().expect("Expected u32."))
        .collect()
}
