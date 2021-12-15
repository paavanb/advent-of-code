use counter::Counter;
use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

const DATA_FILE: &str = "data/polymer_template.txt";

type InsertionRules = HashMap<(char, char), char>;

#[derive(Clone, Debug)]
struct PolymerTemplate {
    char_counts: Counter<char>,
    pair_counts: Counter<(char, char)>,
}

fn main() {
    let template = get_polymer_template(DATA_FILE);
    let insertion_rules = get_pair_insertion_rules(DATA_FILE);
    println!("Part one: {}", get_solution(&template, &insertion_rules, 10));
    println!("Part two: {}", get_solution(&template, &insertion_rules, 40));
}

fn get_solution(template: &PolymerTemplate, rules: &InsertionRules, iterations: u16) -> usize {
    let final_template: PolymerTemplate = (0..iterations)
        .fold(template.clone(), |accum, _| apply_rules(&accum, rules));

    let sorted = final_template.char_counts.most_common();
    return sorted.first().expect("Expected at least one.").1
        - sorted.last().expect("Expected at least one.").1;
}

fn apply_rules(template: &PolymerTemplate, rules: &InsertionRules) -> PolymerTemplate {
    let mut char_counts = template.char_counts.clone();
    let mut pair_counts = Counter::new();

    template.pair_counts.iter().for_each(|(pair, pair_count)| {
        if rules.contains_key(pair) {
            let insertion_char = rules[pair];

            char_counts[&insertion_char] += pair_count;
            pair_counts[&(pair.0, insertion_char)] += pair_count;
            pair_counts[&(insertion_char, pair.1)] += pair_count;
        } else {
            pair_counts[pair] += pair_count;
        }
    });

    return PolymerTemplate {
        char_counts,
        pair_counts,
    };
}

fn get_polymer_template(filename: &str) -> PolymerTemplate {
    let chars = fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .next()
        .expect("Expected at least one line.")
        .to_string()
        .chars()
        .collect::<Vec<_>>();

    let char_counts = chars.clone().into_iter().collect::<Counter<_>>();
    return PolymerTemplate {
        char_counts,
        pair_counts: chars.into_iter().tuple_windows::<(_, _)>().collect::<Counter<_>>(),
    };
}

fn get_pair_insertion_rules(filename: &str) -> InsertionRules {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .split("\n")
        .skip(2)
        .map(|s| {
            let parts = s.split(" ").collect::<Vec<_>>();
            let pair = parts[0].chars().collect::<Vec<_>>();
            let insertion_char = parts[2].chars().collect::<Vec<_>>()[0];

            ((pair[0], pair[1]), insertion_char)
        })
        .collect()
}
