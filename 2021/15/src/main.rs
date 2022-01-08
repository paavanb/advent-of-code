use std::cmp::{Reverse};
use std::collections::{HashMap};
use std::fs;

use priority_queue::PriorityQueue;

const DATA_FILE: &str = "data/chiton_risk.txt";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    let risk_map = get_risk_map(DATA_FILE);
    let tiled_risk_map = get_tiled_risk_map(&risk_map);
    println!("Part one: {}", get_lowest_risk(&risk_map).expect("Expected path to end."));
    println!("Part two: {}", get_lowest_risk(&tiled_risk_map).expect("Expected path to end."));
}

/// Get lowest risk of the path from the top left to the bottom right of the risk map.
/// Uses Dijkstra's algorithm.
fn get_lowest_risk(risk_map: &Vec<Vec<u8>>) -> Option<u32> {
    let mut unvisited_queue: PriorityQueue<Position, Reverse<u32>> = PriorityQueue::new();
    // Map from position to distance
    let mut visited_nodes: HashMap<Position, u32> = HashMap::new();

    let (row_length, col_length) = get_dimensions(&risk_map);

    // Initialize heap with all nodes
    for row in 0..row_length {
        for col in 0..col_length {
            // Skip the starting position, we will consider it first
            if row != 0 || col != 0 {
                unvisited_queue.push(Position { row, col }, Reverse(u32::MAX));
            } else {
                unvisited_queue.push(Position { row, col }, Reverse(0));
            }
        }
    }

    let goal = Position { row: row_length - 1, col: col_length - 1};
    // Iterate until we've visited the final node
    while !visited_nodes.contains_key(&goal) {
        let next = unvisited_queue.pop();

        match next {
            Some((current_pos, Reverse(distance))) => {
                let neighbors: Vec<Position> = get_neighbors(risk_map, &current_pos)
                    .into_iter()
                    .filter(|neighbor| !visited_nodes.contains_key(neighbor)) // Only consider unvisted neighbors
                    .collect();

                neighbors.iter().for_each(|neighbor| {
                    if let Some(&Reverse(neighbor_distance)) = unvisited_queue.get_priority(neighbor) {
                        let candidate_distance = distance + risk_map[neighbor.row][neighbor.col] as u32;
                        if neighbor_distance > candidate_distance {
                            unvisited_queue.change_priority(neighbor, Reverse(candidate_distance));
                        }
                    }
                });

                visited_nodes.insert(current_pos, distance);
            },
            None => {
                return None;
            }
        }
    }

    return Some(visited_nodes.get(&goal).expect("Expected path to end.").clone());
}

fn get_neighbors(risk_map: &Vec<Vec<u8>>, pos: &Position) -> Vec<Position> {
    let mut neighbors = Vec::new();
    let (row_length, col_length) = get_dimensions(risk_map);

    if pos.row > 0 {
        neighbors.push(Position {
            row: pos.row - 1,
            col: pos.col,
        });
    }

    if pos.col > 0 {
        neighbors.push(Position {
            row: pos.row,
            col: pos.col - 1,
        });
    }

    if pos.row < row_length - 1 {
        neighbors.push(Position {
            row: pos.row + 1,
            col: pos.col,
        });
    }

    if pos.col < col_length - 1 {
        neighbors.push(Position {
            row: pos.row,
            col: pos.col + 1,
        });
    }

    return neighbors;
}

fn get_dimensions(risk_map: &Vec<Vec<u8>>) -> (usize, usize) {
    let row_length = risk_map.len();

    if row_length > 0 {
        return (row_length, risk_map[0].len());
    }
    return (0, 0);
}

/// Tile the risk map into a 5x5 grid, incrementing risks appropriately (according to Part Two).
fn get_tiled_risk_map(risk_map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (row_length, col_length) = get_dimensions(risk_map);

    let mut tiled_map = Vec::new();
    for row_idx in 0..(row_length * 5) {
        let mut row = Vec::new();
        for col_idx in 0..(col_length * 5) {
            let orig_row_idx = row_idx % row_length;
            let orig_col_idx = col_idx % col_length;
            let orig_risk = risk_map[orig_row_idx][orig_col_idx] as u8;
            let tile_distance = (row_idx / row_length + col_idx / col_length) as u8;

            let new_risk = ((orig_risk + tile_distance - 1) % 9) + 1;

            row.push(new_risk);
        }

        tiled_map.push(row);
    }

    return tiled_map;
}

fn get_risk_map(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Expected int.") as u8)
                .collect()
        })
        .collect()
}
