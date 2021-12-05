use std::fs;
use std::collections::HashSet;

// Bingo boards are 5x5 grids
type Board = [[u8; 5]; 5];

const DATAFILE: &str = "data/bingo.txt";

fn main() {
    let data = get_data(DATAFILE);
    let boards = parse_boards(&data);
    let draw_numbers = parse_draw_numbers(&data);

    let winning_scores = get_winning_scores(&boards, &draw_numbers[..]);

    let part_one = winning_scores.first().expect("Expected at least one winning board.").clone();
    let part_two = winning_scores.last().expect("Expected at least one winning board.").clone();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

/// Get the winning scores for all boards that win.
fn get_winning_scores(boards: &Vec<Board>, draw_numbers: &[u8]) -> Vec<u32> {
    let mut winning_scores = vec![];
    // Track boards that have won so far
    let mut winning_boards: HashSet<&Board> = HashSet::new();
    for i in 0..draw_numbers.len() {
        let drawn_numbers = &draw_numbers[0..=i];
        let new_winning_boards: Vec<&Board> = boards
            .into_iter()
            .filter(|b| !winning_boards.contains(*b) && has_bingo(b, drawn_numbers))
            .collect();
        winning_boards.extend(new_winning_boards.iter());


        let mut new_winning_scores: Vec<u32> = new_winning_boards.iter().map(|winning_board| {
            let unmarked = get_unmarked(&winning_board, drawn_numbers);
            // Avoid Iterator::sum to prevent overflow
            let sum_unmarked: u32 = unmarked.into_iter().fold(0 as u32, |accum, item| accum + item as u32);
            return sum_unmarked * draw_numbers[i] as u32;
        }).collect();
        winning_scores.append(&mut new_winning_scores);
    }

    return winning_scores;
}

fn has_bingo(board: &Board, drawn_numbers: &[u8]) -> bool {
    // Keep track of the count of drawn numbers per row/col
    let mut drawn_per_row = [0; 5];
    let mut drawn_per_col = [0; 5];

    for &drawn_number in drawn_numbers {
        let location = find_in_board(board, drawn_number);

        if let Some((row, col)) = location {
            drawn_per_row[row] += 1;
            drawn_per_col[col] += 1;
        }
    }

    // There is a bingo if any row or column contains 5 drawn numbers.
    let has_five = |n| n == 5;
    return drawn_per_row.into_iter().any(has_five) || drawn_per_col.into_iter().any(has_five);
}

/// Given a board and a list of drawn numbers, return the unmarked numbers.
fn get_unmarked(board: &Board, drawn_numbers: &[u8]) -> Vec<u8> {
    let marked_numbers: HashSet<u8> = drawn_numbers.iter().map(u8::clone).collect();
    let mut unmarked_numbers = vec![];

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            let value = board[row][col];
            if !marked_numbers.contains(&value) {
                unmarked_numbers.push(value)
            }
        }
    }

    return unmarked_numbers;
}

/// Find the number in the board, if it exists.
/// # Returns
///   `Some((row, col))` if the number exists in the board
///   `None` if the number does not exist in the board
fn find_in_board(board: &Board, number: u8) -> Option<(usize, usize)> {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == number {
                return Some((row, col));
            }
        }
    }

    return None;
}

fn get_data(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(str::to_string)
        .collect()
}

fn parse_draw_numbers(data: &Vec<String>) -> Vec<u8> {
    data[0]
    .split(",")
    .map(|s| s.parse::<u8>().expect("Natural number expected."))
    .collect()
}

fn parse_boards(data: &Vec<String>) -> Vec<Board> {
    // "Chop off" the first two lines, leaving just the board definitions
    let boards_data = &data[2..];

    assert_eq!((boards_data.len() + 1) % 6, 0, "Incorrect number of lines for bingo boards.");

    let num_boards = (boards_data.len() + 1) / 6;

    let mut boards = vec![];
    for i in 0..num_boards {
        let start_index = i * 6;
        let mut board = [[0; 5]; 5];
        for j in start_index..start_index+5 {
            let row = j - start_index;
            let values: Vec<u8> = boards_data[j]
                .split_whitespace()
                .map(|s| s.parse::<u8>().expect("Natural number expected."))
                .collect();

            assert_eq!(values.len(), 5, "Expected five values per bingo board row.");
            for col in 0..5 {
                board[row][col] = values[col];
            }
        }

        boards.push(board);
    }

    return boards;
}
