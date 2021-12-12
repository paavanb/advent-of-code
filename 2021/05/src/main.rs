use std::cmp;
use std::collections::HashSet;
use std::fs;

use num;

const DATA_FILE: &str = "data/vents.txt";

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// Get all the points in this line.
    fn walk(&self) -> Vec<Point> {
        let x_offset = num::signum(self.end.x - self.start.x);
        let y_offset = num::signum(self.end.y - self.start.y);

        let mut points = vec![];
        let mut x = self.start.x;
        let mut y = self.start.y;
        for _ in 0..=self.length() {
            points.push(Point { x, y });
            x += x_offset;
            y += y_offset;
        }

        return points;
    }

    fn length(&self) -> usize {
        return cmp::max(
            (self.start.x as i32 - self.end.x as i32).abs(),
            (self.start.y as i32 - self.end.y as i32).abs(),
        ) as usize;
    }
}

fn main() {
    let lines = get_data(DATA_FILE);
    let axis_aligned_lines: Vec<_> = lines
        .iter()
        .filter(|l| is_axis_aligned(&l))
        .map(Line::clone)
        .collect();
    let part_one_answer = num_overlaps(&axis_aligned_lines);
    let part_two_answer = num_overlaps(&lines);
    println!("Part one: {}", part_one_answer);
    println!("Part two: {}", part_two_answer);
}

fn num_overlaps(lines: &Vec<Line>) -> usize {
    let mut points: HashSet<Point> = HashSet::new();
    let mut intersections: HashSet<Point> = HashSet::new();
    for line in lines {
        for point in line.walk() {
            if points.contains(&point) {
                intersections.insert(point);
            }
            points.insert(point);
        }
    }

    return intersections.len();
}

fn is_axis_aligned(line: &Line) -> bool {
    return line.start.x == line.end.x || line.start.y == line.end.y;
}

fn parse_point(s: &str) -> Point {
    let parts: Vec<_> = s.split(",").collect();
    assert_eq!(parts.len(), 2, "Expected 2D point, got: {}", s);

    let x = parts[0].parse::<i32>().expect("Expected natural number.");
    let y = parts[1].parse::<i32>().expect("Expected natural number.");

    return Point { x, y };
}

fn get_data(filename: &str) -> Vec<Line> {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            let parts: Vec<_> = s.split(" ").collect();
            assert_eq!(parts.len(), 3, "Invalid format, found: {}", s);

            return Line {
                start: parse_point(parts[0]),
                end: parse_point(parts[2]),
            };
        })
        .collect()
}
