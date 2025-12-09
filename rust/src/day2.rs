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

fn parse_input(input: String) -> Vec<(String, String)> {
    let mut parsed: Vec<(String, String)> = Vec::new();

    let file = File::open(input).expect("opening input file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("reading line") != 0 {
        let ranges = line.trim_end().split(',');
        for range in ranges {
            let bounds: Vec<&str> = range.split('-').collect();
            parsed.push((bounds[0].to_owned(), bounds[1].to_owned()));
        }
        line.clear();
    }
    return parsed;
}

fn part_one(parsed: &Vec<(String, String)>) -> u128 {
    let mut count: u128 = 0;
    for range in parsed {
        let lwr = &(range.0).parse::<u128>().expect("");
        let upr = &(range.1)
            .parse::<u128>()
            .expect(&format!("parsing '{}'", range.1));
        for i in *lwr..(upr + 1) {
            let s = i.to_string();
            if s[..(s.len() / 2)] == s[(s.len() / 2)..] {
                count += i;
            }
        }
    }
    return count;
}

fn part_two(parsed: &Vec<(String, String)>) -> u128 {
    let mut count: u128 = 0;
    for range in parsed {
        let lwr = &(range.0).parse::<u128>().expect("");
        let upr = &(range.1)
            .parse::<u128>()
            .expect(&format!("parsing '{}'", range.1));
        for i in *lwr..(upr + 1) {
            let s = i.to_string();
            for j in 1..(s.len() / 2) + 1 {
                if s[..j].repeat(s.len() / j) == s {
                    count += i;
                    break;
                }
            }
        }
    }
    return count;
}

// not complete
fn part_one_opt(parsed: &Vec<(String, String)>) -> u128 {
    let mut count: u128 = 0;
    for range in parsed {
        let l0 = (range.0.len() + 1) / 2;
        let l1 = (range.1.len() + 1) / 2;
        let msb = (&(range.0)[0..l0], &(range.1)[0..l1]);
        println!("{}, {}", msb.0, msb.1);
        let lsb = (&(range.0)[l0..], &(range.1)[l1..]);
        println!("{}, {}", lsb.0, lsb.1);
        /*
         * 7386 7501 1
         * 7363 0108 0
         * msb  lsb
         */
        let d: i32 =
            msb.1.parse::<i32>().expect("msb 1 parse") - msb.0.parse::<i32>().expect("msb 0 parse");
        if msb.0 > lsb.0 && msb.0 < lsb.1 {
            count += (msb.0.to_string() + lsb.0)
                .parse::<u128>()
                .expect("msb 0 parse");
        }
        if msb.0 != msb.1 && msb.1 < lsb.1 && msb.1 > lsb.0 {
            count += (msb.1.to_string() + lsb.1)
                .parse::<u128>()
                .expect("msb 1 parse");
        }
        if d > 2 {
            for i in msb.0.parse::<i32>().expect("") + 1..msb.1.parse::<i32>().expect("") {
                count += (i.to_string() + &i.to_string()).parse::<u128>().expect("");
            }
        }
    }
    return count;
}
