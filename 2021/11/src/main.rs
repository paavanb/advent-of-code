use std::collections::{HashSet, VecDeque};
use std::fs;

const DATA_FILE: &str = "data/dumbo_octopuses.txt";

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    let octopus_state = get_octopuses(DATA_FILE);
    println!("Part one: {}", part_one(&octopus_state));
    println!("Part two: {}", part_two(&octopus_state));
}

fn part_one(octopus_state: &Vec<Vec<u8>>) -> u64 {
    let mut next_state = octopus_state.clone();
    let mut num_flashes = 0;
    for _ in 0..100 {
        next_state = step(&next_state);
        num_flashes += next_state
            .to_owned()
            .into_iter()
            .map(|row| {
                row.iter().fold(0, |sum, &state| match state {
                    0 => sum + 1,
                    _ => sum,
                })
            })
            .sum::<u64>();
    }

    return num_flashes;
}

fn part_two(octopus_state: &Vec<Vec<u8>>) -> u64 {
    let mut next_state = octopus_state.clone();
    let mut step_num = 0;
    let mut all_flashes = false;
    while !all_flashes {
        next_state = step(&next_state);
        all_flashes = next_state
            .to_owned()
            .into_iter()
            .all(|row| row.into_iter().all(|v| v == 0));

        step_num += 1;
    }

    return step_num;
}

fn step(octopus_state: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut next_state = octopus_state.clone();
    let mut flashed: HashSet<Position> = HashSet::new();
    let mut flash_queue: VecDeque<Position> = VecDeque::new();

    // Increment all octpuses state
    for row in 0..next_state.len() {
        for col in 0..next_state[row].len() {
            next_state[row][col] += 1;
            if next_state[row][col] > 9 {
                flash_queue.push_back(Position { row, col });
            }
        }
    }

    // Flash until no new flashes
    while flash_queue.len() > 0 {
        let flashing_pos = flash_queue.pop_front().expect("Unreachable.");
        let neighbors = get_neighbors(&next_state, flashing_pos);

        for neighbor_pos in neighbors {
            if !flashed.contains(&neighbor_pos) && !flash_queue.contains(&neighbor_pos) {
                next_state[neighbor_pos.row][neighbor_pos.col] += 1;
                if next_state[neighbor_pos.row][neighbor_pos.col] > 9 {
                    flash_queue.push_back(neighbor_pos);
                }
            }
        }

        flashed.insert(flashing_pos);
    }

    // Set all flashes to 0
    for Position { row, col } in flashed {
        next_state[row][col] = 0;
    }

    return next_state;
}

fn get_neighbors(octopuses: &Vec<Vec<u8>>, pos: Position) -> Vec<Position> {
    let mut neighbors = vec![];
    let min_row = if pos.row == 0 { 0 } else { pos.row - 1 };
    let max_row = pos.row + 1;
    let min_col = if pos.col == 0 { 0 } else { pos.col - 1 };
    let max_col = pos.col + 1;

    for row in min_row..=max_row.min(octopuses.len() - 1) {
        for col in min_col..=max_col.min(octopuses[row].len() - 1) {
            let neighbor_pos = Position { row, col };
            if neighbor_pos != pos {
                neighbors.push(neighbor_pos);
            }
        }
    }

    return neighbors;
}

fn get_octopuses(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u8>().expect("Expected u8."))
                .collect()
        })
        .collect()
}
