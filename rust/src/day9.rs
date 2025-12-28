use std::{
    collections::VecDeque,
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

fn parse_input(input: String) -> Vec<(u128, u128)> {
    let mut parsed: Vec<(u128, u128)> = Vec::new();

    let file = File::open(input).expect("opening input file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("reading line") != 0 {
        let nums: Vec<&str> = line.trim_end().split(',').collect();
        parsed.push((
            nums[0].parse::<u128>().expect("parsing num"),
            nums[1].parse::<u128>().expect("parsing num"),
        ));
        line.clear();
    }
    return parsed;
}

fn part_one(parsed: &Vec<(u128, u128)>) -> u128 {
    let mut big_area = 0;
    for i in 0..parsed.len() {
        for j in i..parsed.len() {
            let area = (((parsed[i].0 as i128 - parsed[j].0 as i128).abs() + 1)
                * ((parsed[i].1 as i128 - parsed[j].1 as i128).abs() + 1))
                as u128;
            if area > big_area {
                big_area = area;
            }
        }
    }
    return big_area;
}

fn part_two(parsed: &Vec<(u128, u128)>) -> u128 {
    let mut pos_buf: VecDeque<(u128, u128)> = VecDeque::new();
    for ia in 0..parsed.len() + 3 {
        let i = ia.rem_euclid(parsed.len());
    }
    return 0;
}
