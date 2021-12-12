use std::collections::{HashSet, VecDeque};
use std::fs;

const DATA_FILE: &str = "data/cave_heightmap.txt";

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    let heightmap = get_heightmap(DATA_FILE);
    println!("Part one: {}", part_one(&heightmap));
    println!("Part two: {}", part_two(&heightmap));
}

fn part_one(heightmap: &Vec<Vec<u8>>) -> u32 {
    get_local_minima(heightmap)
        .into_iter()
        .map(|p| heightmap[p.row][p.col] as u32 + 1)
        .sum()
}

fn part_two(heightmap: &Vec<Vec<u8>>) -> u32 {
    let local_minima = get_local_minima(heightmap);
    let mut basins: Vec<HashSet<Position>> = vec![];

    for minimum in local_minima {
        // Skip a minimum if it is already contained in an existing basin
        if basins
            .to_owned()
            .into_iter()
            .any(|basin| basin.contains(&minimum))
        {
            continue;
        }

        basins.push(get_basin(heightmap, minimum));
    }

    // Sort high to low
    basins.sort_unstable_by(|basin_1, basin_2| basin_2.len().cmp(&basin_1.len()));

    if basins.len() < 3 {
        panic!("Expected at least three basins.");
    }

    return (basins[0].len() * basins[1].len() * basins[2].len()) as u32;
}

fn get_basin(heightmap: &Vec<Vec<u8>>, position: Position) -> HashSet<Position> {
    let mut basin = HashSet::new();

    // Iterative breadth-first search
    let mut nodes = VecDeque::from([position]);
    while nodes.len() > 0 {
        let node = nodes.pop_back().expect("Unreachable.");
        let neighbors = get_neighbors(heightmap, node);

        let new_neighbors_in_basin = neighbors
            .into_iter()
            .filter(|p| heightmap[p.row][p.col] < 9 && !basin.contains(p))
            .collect::<Vec<_>>();

        basin.extend(new_neighbors_in_basin.clone());
        nodes.extend(new_neighbors_in_basin);
    }

    return basin;
}

fn get_local_minima(heightmap: &Vec<Vec<u8>>) -> Vec<Position> {
    let mut local_minima = vec![];
    for row in 0..heightmap.len() {
        for col in 0..heightmap[0].len() {
            let height = heightmap[row][col];
            let neighbors = get_neighbors(heightmap, Position { row, col });

            if neighbors
                .into_iter()
                .all(|p| heightmap[p.row][p.col] > height)
            {
                local_minima.push(Position { row, col });
            }
        }
    }

    return local_minima;
}

fn get_neighbors(heightmap: &Vec<Vec<u8>>, pos: Position) -> Vec<Position> {
    let mut neighbors = vec![];
    let row = pos.row;
    let col = pos.col;

    if row > 0 {
        neighbors.push(Position {
            row: row - 1,
            col: col,
        });
    }

    if row < heightmap.len() - 1 {
        neighbors.push(Position {
            row: row + 1,
            col: col,
        });
    }

    if col > 0 {
        neighbors.push(Position {
            row: row,
            col: col - 1,
        });
    }

    if col < heightmap[row].len() - 1 {
        neighbors.push(Position {
            row: row,
            col: col + 1,
        });
    }

    return neighbors;
}

fn get_heightmap(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u8>().expect("Expected u8"))
                .collect()
        })
        .collect()
}
