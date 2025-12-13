use std::{
    cmp::{max, min},
    collections::HashSet,
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

fn count_accessible(
    map: &Vec<String>,
    range: i32,
    limit: i32,
    mut seen: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut accessible: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if map[y as usize].chars().nth(x as usize).unwrap() == '@'
                && !seen.contains(&(y as usize, x as usize))
            {
                let mut adj = 0i32;
                for j in max(0, y - range)..min(y + range + 1, height) {
                    for i in max(0, x - range)..min(x + range + 1, width) {
                        if y != j || x != i {
                            if map[j as usize].chars().nth(i as usize).expect("char") == '@'
                                && !seen.contains(&(j as usize, i as usize))
                            {
                                adj += 1;
                            }
                        }
                    }
                }
                if adj < limit {
                    accessible.insert((y as usize, x as usize));
                }
            }
        }
    }
    for p in accessible {
        seen.insert(p);
    }
    return seen;
}

fn part_one(parsed: &Vec<String>) -> u128 {
    return count_accessible(parsed, 1i32, 4i32, HashSet::new()).len() as u128;
}

// Good news! Changing to a HashSet took the running time down to 4931580 from 109491323. That's a
// two orders of magnitude reduction!
fn part_two(parsed: &Vec<String>) -> u128 {
    let mut removed: HashSet<(usize, usize)> = HashSet::new();
    let mut prev_rem = -1isize;
    while prev_rem != removed.len() as isize {
        prev_rem = removed.len() as isize;
        removed = count_accessible(parsed, 1, 4, removed);
    }

    return removed.len() as u128;
}
