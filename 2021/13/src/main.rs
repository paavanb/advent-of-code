use std::fs;
use std::collections::HashSet;

const DATA_FILE: &str = "data/dot_transparency.txt";

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn main() {
    let dot_matrix = get_dot_matrix(DATA_FILE);
    let instructions = get_instructions(DATA_FILE);

    println!("Part one: {}", part_one(&dot_matrix, &instructions));
    println!("Part two:");
    part_two(&dot_matrix, &instructions);
}

fn part_two(matrix: &HashSet<Point>, instructions: &Vec<FoldInstruction>) -> () {
    let final_matrix = instructions.iter().fold(matrix.clone(), |accum, ixn| fold_matrix(&accum, &ixn));

    let max_x = final_matrix.iter().map(|p| p.x).max().expect("Expected at least one point.");
    let max_y = final_matrix.iter().map(|p| p.y).max().expect("Expected at least one point.");

    // Print the matrix
    for y in 0..=max_y {
        for x in 0..=max_x {
            if final_matrix.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part_one(matrix: &HashSet<Point>, instructions: &Vec<FoldInstruction>) -> usize {
    fold_matrix(matrix, instructions.first().expect("Expected at least one instruction.")).len()
}

fn fold_matrix(matrix: &HashSet<Point>, instruction: &FoldInstruction) -> HashSet<Point> {
    let mut new_matrix = HashSet::new();

    for point in matrix {
        new_matrix.insert(
            match instruction {
                &FoldInstruction::Y(offset) =>
                    Point {
                        x: point.x,
                        y: if point.y > offset { offset - (point.y - offset) } else { point.y },
                    },
                &FoldInstruction::X(offset) =>
                    Point {
                        x: if point.x > offset { offset - (point.x - offset) } else { point.x },
                        y: point.y,
                    }
            }
        );
    }

    return new_matrix;
}

fn get_dot_matrix(filename: &str) -> HashSet<Point> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .take_while(|&s| s.len() > 0) // Dots and instructions are separated by an empty line
        .map(|s| {
            let parts = s
                .split(",")
                .map(|s| s.parse::<usize>().expect("Expected int."))
                .collect::<Vec<_>>();
            Point {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect()
}

fn get_instructions(filename: &str) -> Vec<FoldInstruction> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .skip_while(|&s| s.len() > 0) // Dots and instructions are separated by an empty line
        .skip(1) // Skip the empty line
        .map(|s| {
            let fold_instr = s.split(" ").collect::<Vec<_>>()[2];
            let parts = fold_instr.split("=").collect::<Vec<_>>();
            let offset = parts[1].parse::<usize>().expect("Expected int.");

            match parts[0] {
                "x" => FoldInstruction::X(offset),
                "y" => FoldInstruction::Y(offset),
                _ => panic!("Invalid axis encountered."),
            }
        })
        .collect()
}
