use std::{
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
    let mut count: u128 = 0;
    for line in parsed {
        let mut i = 0;
        let mut j = 1;
        for x in 1..line.len() {
            let c = line.chars().nth(x).unwrap();
            if line.chars().nth(i).unwrap() < c && x < line.len() - 1 {
                i = x;
                j = i + 1;
            } else if line.chars().nth(j).unwrap() < c {
                j = x;
            }
        }
        let mut num = String::new();
        num.push(line.chars().nth(i).unwrap());
        num.push(line.chars().nth(j).unwrap());
        count += num.parse::<u128>().unwrap();
    }
    return count;
}

fn part_two(parsed: &Vec<String>) -> u128 {
    let mut count: u128 = 0;
    return count;
}
