use std::{collections::HashSet, convert::TryInto, fs};

#[derive(Debug)]
struct LineEntry<'a> {
    // Different ways the submarine tries to render the digit
    signal_patterns: [&'a str; 10],
    digit_output_values: [&'a str; 4],
}

fn solve_day8_p1(lines: &Vec<LineEntry>) -> usize {
    // Times only 1, 4, 7, 8 appeared on the output values
    let mut count = 0;
    for line in lines {
        let LineEntry {
            digit_output_values,
            ..
        } = line;
        let nums_in_output = digit_output_values
            .iter()
            .filter(|s| match s.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count();
        count += nums_in_output;
    }
    count
}

fn decode(s: &str, one_set: &HashSet<char>, four_set: &HashSet<char>) -> char {
    // Uses the new logic provided in the question
    match s.len() {
        2 => '1',
        4 => '4',
        3 => '7',
        7 => '8',
        len => {
            let charset: HashSet<char> = s.chars().collect();
            match (
                len,
                one_set.difference(&charset).count(),
                four_set.difference(&charset).count(),
            ) {
                // 5 -> 2, 3, 5
                (5, 0, _) => '3',
                (5, 1, 2) => '2',
                (5, _, _) => '5',
                // 6 -> 0, 6, 9
                (6, 1, 1) => '6',
                (6, _, 1) => '0',
                (6, _, 0) => '9',
                _ => panic!("Unexpected number of chars"),
            }
        }
    }
}

fn solve_day8_p2(lines: &Vec<LineEntry>) -> usize {
    // Times only 1, 4, 7, 8 appeared on the output values

    let mut count = 0;
    for line in lines {
        let LineEntry {
            digit_output_values,
            signal_patterns,
        } = line;
        let one_set: HashSet<char> = signal_patterns
            .iter()
            .find(|s| s.len() == 2)
            .unwrap()
            .chars()
            .collect();
        let four_set: HashSet<char> = signal_patterns
            .iter()
            .find(|s| s.len() == 4)
            .unwrap()
            .chars()
            .collect();

        let nums_in_output: usize = digit_output_values
            .iter()
            .map(|&s| decode(s, &one_set, &four_set))
            .collect::<String>()
            .parse()
            .unwrap();

        count += nums_in_output;
    }
    count
}

fn main() {
    let file = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let entries: Vec<LineEntry> = file
        .lines()
        .map(|line| {
            let mut split_iter = line.splitn(2, "|");
            let signal_patterns = split_iter
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let digit_output_values = split_iter
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            LineEntry {
                signal_patterns,
                digit_output_values,
            }
        })
        .collect();

    println!("{:?}", solve_day8_p1(&entries));
    println!("{:?}", solve_day8_p2(&entries));
}
