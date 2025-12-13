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

fn max_char(line: &String, lwr: usize, upr: usize) -> usize {
    let mut loc = lwr;
    for i in lwr..upr {
        if line.chars().nth(i).unwrap() > line.chars().nth(loc).unwrap() {
            loc = i;
        }
    }
    return loc;
}

fn get_num(line: &String, digits: usize) -> u128 {
    let mut n = 0u128;
    let mut lwr = 0usize;
    let mut upr = line.len() - digits + 1;
    for i in 0..digits {
        let loc = max_char(line, lwr, upr);
        n += line
            .chars()
            .nth(loc)
            .expect("char loc")
            .to_digit(10)
            .expect("digit conversion") as u128
            * 10u128.pow((digits - i - 1) as u32);
        lwr = loc + 1;
        upr += 1;
    }
    return n;
}

fn part_one(parsed: &Vec<String>) -> u128 {
    let mut count: u128 = 0;
    for line in parsed {
        let x = get_num(line, 2usize);
        count += x;
    }
    return count;
}

fn part_two(parsed: &Vec<String>) -> u128 {
    let mut count: u128 = 0;
    for line in parsed {
        let x = get_num(line, 12usize);
        count += x;
    }
    return count;
}
