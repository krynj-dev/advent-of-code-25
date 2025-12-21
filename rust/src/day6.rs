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
        parsed.push(line.trim_matches(|c| c == '\n').to_owned());
        line.clear();
    }
    return parsed;
}

fn read_line(line: &String) -> Vec<u128> {
    let mut vals: Vec<u128> = Vec::new();
    let mut bfr = String::new();
    let chrs: Vec<char> = line.chars().collect();
    for i in 0..chrs.len() {
        let c = chrs[i];
        if c == ' ' && bfr.len() > 0 {
            vals.push(bfr.parse::<u128>().expect("parsing num"));
            bfr.clear();
        } else if c != ' ' {
            bfr.push(c);
        }
    }
    if bfr.len() > 0 {
        vals.push(
            bfr.parse::<u128>()
                .expect(&format!("parsing num '{}'", bfr)),
        );
    }
    return vals;
}

fn part_one(parsed: &Vec<String>) -> u128 {
    let mut ops: Vec<char> = Vec::new();
    let mut totals = read_line(&parsed[0]);
    for c in parsed[parsed.len() - 1].chars() {
        if c != ' ' {
            ops.push(c);
        }
    }
    for line in &parsed[1..parsed.len() - 1] {
        let vals = read_line(line);
        for i in 0..vals.len() {
            if ops[i] == '+' {
                totals[i] += vals[i];
            } else if ops[i] == '*' {
                totals[i] *= vals[i];
            }
        }
    }
    return totals.iter().sum();
}

fn part_two(parsed: &Vec<String>) -> u128 {
    let ops: Vec<char> = parsed[parsed.len() - 1].chars().collect();
    let mut totals: Vec<u128> = Vec::new();
    let mut i = 0usize;
    let mut t = 0usize;
    while i < parsed[0].len() {
        let mut bfr = String::new();
        let mut i2 = i;
        while i2 < parsed[0].len() {
            for j in 0..parsed.len() - 1 {
                let a: Vec<char> = parsed[j].chars().collect();
                let c = a[i2];
                if c != ' ' {
                    bfr.push(c);
                }
            }
            if bfr.len() > 0 {
                let num = bfr.parse::<u128>().expect("parsing num");
                if ops[i] == '*' {
                    if t == totals.len() {
                        totals.push(1);
                    }
                    totals[t] *= num;
                } else if ops[i] == '+' {
                    if t == totals.len() {
                        totals.push(0);
                    }
                    totals[t] += num;
                }
                bfr.clear();
            } else {
                i = i2;
                t += 1;
                break;
            }
            i2 += 1;
        }
        if i2 == parsed[0].len() {
            break;
        }
        i += 1;
    }
    return totals.iter().sum();
}
