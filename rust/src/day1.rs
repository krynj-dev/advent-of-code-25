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

fn parse_input(input: String) -> Vec<i32> {
    let mut parsed: Vec<i32> = Vec::new();

    let file = File::open(input).expect("Opening day 1 txt");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("reading line") != 0 {
        let sign = if line.chars().nth(0).expect("Reading first char in line") == 'R' {
            1
        } else {
            -1
        };
        let num = line.get(1..).expect("slicing line");
        parsed.push(num.trim_end().parse::<i32>().unwrap() * sign);
        line.clear();
    }
    return parsed;
}

fn part_one(parsed: &Vec<i32>) -> u128 {
    let max = 100;
    let mut pos = 50;
    let mut count = 0;

    for turn in parsed {
        pos = (pos + turn).rem_euclid(max);
        if pos == 0 {
            count += 1;
        }
    }
    return count;
}

fn part_two(parsed: &Vec<i32>) -> u128 {
    let max = 100;
    let mut pos = 50;
    let mut count: u128 = 0;

    for turn in parsed {
        let q: u128 = (turn / max).abs().try_into().unwrap();
        count += q;
        let new_pos = (pos + turn).rem_euclid(max);
        if new_pos == 0
            || pos != 0 && ((*turn > 0 && new_pos < pos) || (*turn < 0 && new_pos > pos))
        {
            count += 1;
        }
        pos = new_pos;
    }
    return count;
}
