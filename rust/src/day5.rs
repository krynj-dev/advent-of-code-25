use std::{
    cmp::{max, min},
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

fn parse_ranges_naive(parsed: &Vec<String>) -> Vec<(u128, u128)> {
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    for line in parsed {
        if line.len() == 0 {
            break;
        }
        let splits: Vec<&str> = line.split('-').collect();
        ranges.push((
            splits[0].parse::<u128>().expect("parsing split lwr"),
            splits[1].parse::<u128>().expect("parsing split upr"),
        ));
    }
    ranges.sort();
    return ranges;
}

fn is_fresh_naive(ranges: &Vec<(u128, u128)>, id: &u128) -> bool {
    for range in ranges {
        if *id >= range.0 && *id <= range.1 {
            return true;
        }
    }
    return false;
}

/*
* The naive solution of this problem (part 1 at least) would be O(mn) time complexity.
* i.e. for each ID n, check if it's within any range m.
* With this preprocessing (that has it's own time complexity of O(m)) we can then use a
* modified binary search on each n which from memory has a time complexity of O(log(n)).
* Bringing this together we can get a final time complexity of O(m + nlog(n)) for part 1.
* Naive had a run of ~1700. This version had a run of ~800
*/
fn parse_ranges(parsed: &Vec<String>) -> Vec<u128> {
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    for line in parsed {
        if line.len() == 0 {
            break;
        }
        let splits: Vec<&str> = line.split('-').collect();
        ranges.push((
            splits[0].parse::<u128>().expect("parsing split lwr"),
            splits[1].parse::<u128>().expect("parsing split upr"),
        ));
    }
    ranges.sort();
    let mut i: usize = 0;
    while i < ranges.len() - 1 {
        let cur = ranges[i];
        let nxt = ranges[i + 1];
        if cur.1 + 1 >= nxt.0 {
            ranges.remove(i + 1);
            ranges[i] = (min(cur.0, nxt.0), max(cur.1, nxt.1));
        } else {
            i += 1;
        }
    }
    let mut rangez: Vec<u128> = Vec::new();
    for range in ranges {
        rangez.push(range.0);
        rangez.push(range.1);
    }
    return rangez;
}

fn is_fresh(ranges: &Vec<u128>, id: &u128) -> bool {
    let mut lo = 0isize;
    let mut hi = ranges.len() as isize;
    while hi - lo >= 0 {
        if (hi - lo) <= 1 {
            return *id > ranges[lo as usize] && (hi - lo == 0 || lo % 2 == 0);
        }
        let mid = lo + ((hi - lo) / 2);
        if *id < ranges[mid as usize] {
            hi = mid;
        } else if *id >= ranges[mid as usize] {
            lo = mid;
        }
    }
    println!("whoops");
    return false;
}

fn part_one(parsed: &Vec<String>) -> u128 {
    let mut count = 0;
    let ranges = parse_ranges(parsed);
    let mut rdy = false;
    for line in parsed {
        if !rdy {
            if line.len() == 0 {
                rdy = true;
            }
            continue;
        }
        let id = line.parse::<u128>().expect("parsing id");
        if is_fresh(&ranges, &id) {
            count += 1;
        }
    }
    return count;
}

fn part_two(parsed: &Vec<String>) -> u128 {
    let ranges = parse_ranges(parsed);
    let mut count = 0;
    let mut i = 0usize;
    while i < ranges.len() {
        count += ranges[i + 1] - ranges[i] + 1;
        i += 2;
    }
    return count;
}
