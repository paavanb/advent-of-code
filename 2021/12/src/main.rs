use std::collections::HashMap;
use std::fs;

const DATA_FILE: &str = "data/cave_map.txt";

type CaveMap = HashMap<String, Vec<String>>;

fn main() {
    let map = get_cave_map(DATA_FILE);
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
}

fn part_one(map: &CaveMap) -> u32 {
    get_paths(map, "start", "end").len() as u32
}

fn part_two(map: &CaveMap) -> u32 {
    get_paths_with_time(map, "start", "end").len() as u32
}

fn get_paths(map: &CaveMap, start: &str, end: &str) -> Vec<Vec<String>> {
    get_subpaths(map, vec![start.to_string()], end.to_string())
}

fn get_paths_with_time(map: &CaveMap, start: &str, end: &str) -> Vec<Vec<String>> {
    get_subpaths_with_time(map, vec![start.to_string()], end.to_string())
}

/// Find all paths to `end` in the cave map with the given path prefix.
fn get_subpaths(map: &CaveMap, path_prefix: Vec<String>, end: String) -> Vec<Vec<String>> {
    let last_cave = path_prefix
        .last()
        .expect("Expected a path of at least length one.");
    let dest_caves = map.get(last_cave).unwrap_or(&vec![]).clone();

    let mut paths = Vec::new();
    for dest in dest_caves {
        let is_small = dest.to_lowercase() == dest;
        if !is_small || !path_prefix.contains(&dest) {
            let new_prefix = {
                let mut tmp = path_prefix.clone();
                tmp.push(dest.clone());
                tmp
            };
            if dest == end {
                paths.push(new_prefix);
            } else {
                let subpaths = get_subpaths(map, new_prefix, end.clone());
                paths.extend(subpaths);
            }
        }
    }

    return paths;
}

/// Find all paths to `end` in the cave map with the given path prefix.
/// With extra time, one small cave per path can be visited twice, except for `start` and `end`
fn get_subpaths_with_time(
    map: &CaveMap,
    path_prefix: Vec<String>,
    end: String,
) -> Vec<Vec<String>> {
    let start_cave = path_prefix
        .first()
        .expect("Expected a path of at least length 1.")
        .clone();
    let last_cave = path_prefix
        .last()
        .expect("Expected a path of at least length one.")
        .clone();
    let dest_caves = map.get(&last_cave).unwrap_or(&vec![]).clone();

    // Count occurence of each cave in the prefix
    let cave_counts = path_prefix
        .clone()
        .iter()
        .fold(HashMap::new(), |mut accum, cave| {
            *accum.entry(cave.clone()).or_insert(0) += 1;
            accum
        });
    // Whether we have visited some small cave twice already
    let have_double_visited = cave_counts
        .iter()
        .any(|(cave, val)| *val > 1 && is_small(&cave));

    let mut paths = Vec::new();
    for dest in dest_caves {
        let num_visits = cave_counts.get(&dest).unwrap_or(&0).clone();

        if !is_small(&dest)
            || !path_prefix.contains(&dest)
            || (!have_double_visited && num_visits == 1 && dest != start_cave && dest != end)
        {
            let new_prefix = {
                let mut tmp = path_prefix.clone();
                tmp.push(dest.clone());
                tmp
            };
            if dest == end {
                paths.push(new_prefix);
            } else {
                let subpaths = get_subpaths_with_time(map, new_prefix, end.clone());
                paths.extend(subpaths);
            }
        }
    }

    return paths;
}

fn is_small(cave: &String) -> bool {
    return cave.to_lowercase() == cave.clone();
}

fn get_cave_map(filename: &str) -> CaveMap {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            let parts = s.split("-").collect::<Vec<_>>();
            let (src, dest) = (parts[0].to_string(), parts[1].to_string());
            vec![(src.clone(), dest.clone()), (dest.clone(), src.clone())]
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (src, dest)| {
            acc.entry(src).or_insert(vec![]).push(dest);
            acc
        })
}
