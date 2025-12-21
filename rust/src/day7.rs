use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::utils::{self, DayResult, PartResult, time_it};

pub fn run(input: String) -> utils::DayResult {
    let parsed = parse_input(input);
    let mut result_one: u128 = 0;
    let time_one = time_it(|| {
        result_one = part_one(&parsed);
    });

    let mut result_two: u128 = 0;
    let time_two = time_it(|| {
        result_two = part_two(&parsed);
    });

    return DayResult(
        PartResult(result_one, time_one.as_micros()),
        PartResult(result_two, time_two.as_micros()),
    );
}

fn parse_input(input: String) -> Vec<String> {
    let mut parsed: Vec<String> = Vec::new();

    let file = File::open(input).expect("opening input file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("reading line") != 0 {
        parsed.push(line.trim_end().to_owned());
        line.clear();
    }
    return parsed;
}

fn part_one(parsed: &Vec<String>) -> u128 {
    let mut count = 0u128;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(parsed[0].find('S').expect("Finding start"));
    for i in 1..parsed.len() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        let line: Vec<char> = parsed[i].chars().collect();
        for j in &beams {
            if line[*j] == '^' {
                new_beams.insert(j - 1);
                new_beams.insert(j + 1);
                count += 1;
            } else {
                new_beams.insert(*j);
            }
        }
        beams = new_beams;
    }
    return count;
}

fn part_two(parsed: &Vec<String>) -> u128 {
    let mut beams: HashMap<usize, u128> = HashMap::new();
    beams.insert(parsed[0].find('S').expect("Finding start"), 1);
    for i in 1..parsed.len() {
        let mut new_beams: HashMap<usize, u128> = HashMap::new();
        let line: Vec<char> = parsed[i].chars().collect();
        for j in beams.keys() {
            if line[*j] == '^' {
                let asplit = &(j - 1);
                let bsplit = &(j + 1);
                if new_beams.contains_key(asplit) {
                    new_beams.insert(*asplit, new_beams[asplit] + beams[j]);
                } else {
                    new_beams.insert(*asplit, beams[j]);
                }
                if new_beams.contains_key(bsplit) {
                    new_beams.insert(*bsplit, new_beams[bsplit] + beams[j]);
                } else {
                    new_beams.insert(*bsplit, beams[j]);
                }
            } else {
                if new_beams.contains_key(j) {
                    new_beams.insert(*j, new_beams[j] + beams[j]);
                } else {
                    new_beams.insert(*j, beams[j]);
                }
            }
        }
        beams = new_beams;
    }
    return beams.values().sum();
}
