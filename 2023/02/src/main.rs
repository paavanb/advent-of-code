use std::cmp;
use std::fs;

// (# red, # green, # blue)
type Game = Vec<(u32, u32, u32)>;

fn main() {
    let games = get_games();
    println!("Part 1: {}", part_1(&games));
    println!("Part 2: {}", part_2(&games));
}

fn part_1(games: &Vec<Game>) -> usize {
    // Find all games possible with only 12 red cubes, 13 green cubes, and 14 blue cubes
    let limit = (12, 13, 14);
    return games
        .iter()
        .enumerate()
        .filter(|(_idx, game)| {
            for round in game.iter() {
                if round.0 > limit.0 || round.1 > limit.1 || round.2 > limit.2 {
                    return false;
                }
            }
            return true;
        })
        .map(|(idx, _game)| idx + 1) // Game ID's are 1-indexed
        .sum();
}

fn part_2(games: &Vec<Game>) -> u32 {
    return games
        .iter()
        .map(|game| get_power(&get_minimum_set(game)))
        .sum();
}

fn get_power(set: &(u32, u32, u32)) -> u32 {
    return set.0 * set.1 * set.2;
}

fn get_minimum_set(game: &Game) -> (u32, u32, u32) {
    return game
        .iter()
        .cloned()
        .reduce(|min_set, item| {
            (
                cmp::max(min_set.0, item.0),
                cmp::max(min_set.1, item.1),
                cmp::max(min_set.2, item.2),
            )
        })
        .unwrap();
}

fn get_games() -> Vec<Game> {
    let lines = get_lines();
    return lines.iter().map(|line| parse_game(line)).collect();
}

fn parse_game(line: &str) -> Game {
    let colon_index = line.find(":").unwrap();
    let rounds_section = line[colon_index + 1..].to_string();
    let rounds = rounds_section.split(";");
    return rounds.map(|round| parse_round(round)).collect();
}

fn parse_round(round: &str) -> (u32, u32, u32) {
    let results = round.split(",");
    return results
        .map(|result| parse_result(result))
        .fold((0, 0, 0), |acc, val| {
            (acc.0 + val.0, acc.1 + val.1, acc.2 + val.2)
        });
}

fn parse_result(result: &str) -> (u32, u32, u32) {
    let parts: Vec<&str> = result.trim_start().split(' ').collect();
    let amount: u32 = parts[0].parse().unwrap();
    let color = parts[1];

    if color == "red" {
        return (amount, 0, 0);
    } else if color == "green" {
        return (0, amount, 0);
    } else {
        return (0, 0, amount);
    }
}

fn get_lines() -> Vec<String> {
    let contents = fs::read_to_string("data/cube_games.txt").expect("Something went wrong.");
    let lines = contents.trim_end().lines().map(String::from).collect();

    return lines;
}
