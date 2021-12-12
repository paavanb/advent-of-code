use std::fs;

const BIT_LENGTH: u8 = 12;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    let report = get_report();
    let mut epsilon_bits: Vec<char> = vec![];

    for i in 0..BIT_LENGTH {
        if has_more_ones(&report, i as usize) {
            epsilon_bits.push('1');
        } else {
            epsilon_bits.push('0')
        }
    }

    let gamma_bits: Vec<char> = epsilon_bits
        .iter()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!("Unexpected character found '{}'", c),
        })
        .collect();

    let epsilon = bitstring_to_value(epsilon_bits);
    let gamma = bitstring_to_value(gamma_bits);

    return epsilon * gamma;
}

fn part_two() -> u32 {
    let report = get_report();

    let oxygen_rating = bitstring_to_value(calculate_rating(&report, true));
    let co2_rating = bitstring_to_value(calculate_rating(&report, false));

    return oxygen_rating * co2_rating;
}

/// Calculate the rating value by progressively filtering down report values.
///
/// # Arguments
/// * `keep_most_common` - Whether to keep values with the most common bit, or not.
///
fn calculate_rating(report: &Vec<Vec<char>>, keep_most_common: bool) -> Vec<char> {
    let mut rating_candidates = report.clone();
    let mut i: usize = 0;
    while i < BIT_LENGTH as usize && rating_candidates.len() > 1 {
        let more_ones = has_more_ones(&rating_candidates, i);
        let keep_ones = (more_ones && keep_most_common) || (!more_ones && !keep_most_common);

        rating_candidates = rating_candidates
            .into_iter()
            .filter(|bits| {
                if keep_ones {
                    bits[i] == '1'
                } else {
                    bits[i] == '0'
                }
            })
            .collect();

        i += 1;
    }

    assert_eq!(
        rating_candidates.len(),
        1,
        "Expected only one candidate to remain."
    );

    return rating_candidates[0].clone();
}

fn has_more_ones(report: &Vec<Vec<char>>, bit_index: usize) -> bool {
    let num_ones: i32 = report.iter().fold(0, |num_ones, bits| {
        let char = bits[bit_index];
        match char {
            '1' => num_ones + 1,
            '0' => num_ones,
            _ => unreachable!("Unrecognized character found '{}'", char),
        }
    });

    return num_ones >= report.len() as i32 - num_ones;
}

fn bitstring_to_value(bitstring: Vec<char>) -> u32 {
    bitstring.iter().fold(0, |epsilon, c| match c {
        '1' => epsilon * 2 + 1,
        '0' => epsilon * 2,
        _ => unreachable!("Unrecognized character found '{}'", c),
    })
}

/**
 * Grab report numbers from puzzle input file.
 */
fn get_report() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("data/diagnostic_report.txt").expect("Something went wrong.");
    let report: Vec<_> = contents
        .trim_end()
        .split("\n")
        .map(|s| str::to_string(s).chars().collect::<Vec<_>>())
        .collect();

    return report;
}
