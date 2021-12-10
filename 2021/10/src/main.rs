use std::fs;
use std::collections::{HashSet, HashMap};

const DATA_FILE: &str = "data/navigation_subsystem.txt";

fn main() {
    let lines = get_subsystem_data(DATA_FILE);
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

fn part_one(lines: &Vec<Vec<char>>) -> u32 {
    let score_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    lines.into_iter().fold(0, |sum, line| {
        match parse_line(line) {
            (_, Some(illegal_char)) => sum + score_map.get(&illegal_char)
                .expect("Expected score mapping for illegal char."),
            (_, None) => sum,
        }
    })
}

fn part_two(lines: &Vec<Vec<char>>) -> u64 {
    let score_map = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut scores = lines.into_iter()
        .map(|line| {
            match parse_line(line) {
                (stack, None) => {
                    Some(
                        stack.iter().rev().fold(0 as u64, |score, curr_char|
                            5 * score + score_map.get(&curr_char).expect("Expected score for char")
                        )
                    )
                },
                (_, Some(_)) => None,
            }
        })
        .filter(|score| score.is_some())
        .map(|score| score.unwrap())
        .collect::<Vec<_>>();
    scores.sort();

    return scores[scores.len() / 2];
}

/// Parses a line, stopping at the first illegal character
/// # Returns
/// (stack, illegal_char)
///     stack - The incomplete parse stack
///     illegal_char - The illegal character, if any
fn parse_line(line: &Vec<char>) -> (Vec<char>, Option<char>) {
    let opening_chars: HashSet<char> = HashSet::from(['(', '{', '[', '<']);
    let closing_chars: HashSet<char> = HashSet::from([')', '}', ']', '>']);

    let mut char_stack: Vec<char> = Vec::new();

    for line_char in line {
        if opening_chars.contains(&line_char) {
            char_stack.push(line_char.clone());
        } else if closing_chars.contains(&line_char) {
            let matching_open = match line_char {
                ')' => '(',
                '}' => '{',
                ']' => '[',
                '>' => '<',
                _ => panic!("Unexpected char found: '{}'", line_char),
            };
            let opt_last_char = char_stack.last();

            match opt_last_char {
                Some(&last_char) => {
                    if last_char != matching_open {
                        return (char_stack, Some(line_char.clone()))
                    }
                    char_stack.pop();
                }
                None => return (char_stack, Some(line_char.clone()))
            }
        } else {
            panic!("Unexpected character found: '{}'", line_char);
        }
    }

    return (char_stack, None);
}

fn get_subsystem_data(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            str::to_string(s).chars().collect()
        })
        .collect()
}
