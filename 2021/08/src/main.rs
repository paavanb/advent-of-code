use std::fs;
use std::collections::HashSet;

const DATA_FILE: &str = "data/segments.txt";

fn main() {
    println!("Part one: {}", part_one(DATA_FILE));
    println!("Part two: {}", part_two(DATA_FILE));
}

fn part_one(filename: &str) -> u16 {
    let outputs = get_outputs(filename);

    outputs.into_iter().flatten().fold(0, |accum, item| {
        match item.len() {
            // Corresponds to displays 1, 4, 7, and 8
            2 | 4 | 3 | 7 => accum + 1,
            _ => accum,
        }
    })
}

fn part_two(filename: &str) -> u32 {
    let signal_entries = get_signals(filename);
    let outputs = get_outputs(filename);
    let mut sum = 0;
    for (signals, outputs) in signal_entries.iter().zip(outputs) {
        let decoder_ring = decode(&signals);

        let signal_values = outputs.iter().map(|signal|
            decoder_ring.to_owned().into_iter().position(|decoded| decoded == signal.clone())
                .expect("Expected a matching signal.")
        ).collect::<Vec<_>>();

        let value = signal_values.into_iter().fold(0, |accum, v| accum * 10 + v);
        sum += value;
    }

    return sum as u32;
}

fn decode(signals: &Vec<HashSet<char>>) -> Vec<HashSet<char>> {
    // 2, 3, or 5
    let five_segment_signals: Vec<_> = signals.iter().filter(|segments| segments.len() == 5).collect();
    // 6, 9, or 0
    let six_segment_signals: Vec<_> = signals.iter().filter(|segments| segments.len() == 6).collect();

    let one = signals.iter().find(|segments| segments.len() == 2).expect("Expected a signal corresponding to 1");
    let four = signals.iter().find(|segments| segments.len() == 4).expect("Expected a signal corresponding to 1");
    let seven = signals.iter().find(|segments| segments.len() == 3).expect("Expected a signal corresponding to 1");
    let eight = signals.iter().find(|segments| segments.len() == 7).expect("Expected a signal corresponding to 1");

    let a_segment = seven.difference(one).map(char::clone).collect::<HashSet<_>>();

    let &nine = six_segment_signals.iter()
        .find(|segments|
            segments
                .difference(&four.union(&a_segment).map(char::clone).collect::<HashSet<char>>())
                .collect::<Vec<_>>().len() == 1
        ).expect("Expected to find signal corresponding to nine.");

    let &two = five_segment_signals.iter()
        .find(|segments|
            segments.difference(nine).collect::<Vec<_>>().len() == 1
        ).expect("Expected to find signal corresponding to two.");

    let &three = five_segment_signals.iter()
        .find(|segments|
            segments.difference(two).collect::<Vec<_>>().len() == 1
        ).expect("Expected to find signal corresponding to three.");

    let &five = five_segment_signals.iter()
        .find(|segments|
            segments.difference(two).collect::<Vec<_>>().len() == 2
        ).expect("Expected to find signal corresponding to five.");

    let &zero = six_segment_signals.iter()
        .find(|segments|
            segments.difference(five).collect::<Vec<_>>().len() == 2
        ).expect("Expected to find signal corresponding to zero.");

    let &six = six_segment_signals.iter()
        .find(|segments|
            segments.difference(nine).collect::<Vec<_>>().len() != 0 &&
            segments.difference(zero).collect::<Vec<_>>().len() != 0
        ).expect("Expected to find signal corresponding to six.");

    return vec![zero, one, two, three, four, five, six, seven, eight, nine]
        .into_iter()
        .map(HashSet::clone)
        .collect();
}

fn get_signals(filename: &str) -> Vec<Vec<HashSet<char>>> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            let parts: Vec<_> = s.split(" | ").collect();
            let outputs = parts[0];
            outputs.split(" ").map(|s| str::to_string(s).chars().collect::<HashSet<_>>()).collect()
        })
        .collect()
}

fn get_outputs(filename: &str) -> Vec<Vec<HashSet<char>>> {
    fs::read_to_string(filename).expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .map(|s| {
            let parts: Vec<_> = s.split(" | ").collect();
            let outputs = parts[1];
            outputs.split(" ").map(|s| str::to_string(s).chars().collect::<HashSet<_>>()).collect()
        })
        .collect()
}
