use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
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

fn parse_input(input: String) -> Vec<(u128, u128, u128)> {
    let mut parsed: Vec<(u128, u128, u128)> = Vec::new();

    let file = File::open(input).expect("opening input file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line).expect("reading line") != 0 {
        let nums: Vec<&str> = line.trim_end().split(',').collect();
        parsed.push((
            nums[0].parse::<u128>().expect("parsing num"),
            nums[1].parse::<u128>().expect("parsing num"),
            nums[2]
                .parse::<u128>()
                .expect(&format!("parsing num {}", nums[2])),
        ));
        line.clear();
    }
    return parsed;
}

fn distance(a: (u128, u128, u128), b: (u128, u128, u128)) -> u128 {
    let xd = (a.0 as i128 - b.0 as i128).abs() as u128;
    let yd = (a.1 as i128 - b.1 as i128).abs() as u128;
    let zd = (a.2 as i128 - b.2 as i128).abs() as u128;
    return xd.pow(2) + yd.pow(2) + zd.pow(2);
}

fn part_one(parsed: &Vec<(u128, u128, u128)>) -> u128 {
    let mut dist_heap: BinaryHeap<Reverse<(u128, (u128, u128, u128), (u128, u128, u128))>> =
        BinaryHeap::new();
    for i in 0..parsed.len() - 1 {
        for j in i + 1..parsed.len() {
            let dist = distance(parsed[i], parsed[j]);
            dist_heap.push(Reverse((dist, parsed[i], parsed[j])));
        }
    }
    let mut n = 0;
    let mut groups: HashMap<(u128, u128, u128), u128> = HashMap::new();
    let mut group_count: HashMap<u128, u128> = HashMap::new();
    for _ in 0..1000 {
        let it = dist_heap.pop().unwrap();
        let gina = groups.contains_key(&it.0.1);
        let ginb = groups.contains_key(&it.0.2);
        if gina && !ginb {
            groups.insert(it.0.2, groups[&it.0.1]);
            group_count.insert(groups[&it.0.1], group_count[&groups[&it.0.1]] + 1);
        }
        if !gina && ginb {
            groups.insert(it.0.1, groups[&it.0.2]);
            group_count.insert(groups[&it.0.2], group_count[&groups[&it.0.2]] + 1);
        }
        if !gina && !ginb {
            groups.insert(it.0.1, n);
            groups.insert(it.0.2, n);
            group_count.insert(n, 2);
            n += 1;
        }
        if gina && ginb && groups[&it.0.1] != groups[&it.0.2] {
            let old_group = groups[&it.0.2];
            let new_group = groups[&it.0.1];
            let mut to_update: Vec<(u128, u128, u128)> = Vec::new();
            for node in groups.keys() {
                if old_group == groups[node] {
                    to_update.push(*node);
                    group_count.insert(new_group, group_count[&new_group] + 1);
                    group_count.insert(old_group, group_count[&old_group] - 1);
                }
            }
            for x in to_update {
                groups.insert(x, new_group);
            }
        }
    }
    let mut vals: Vec<&u128> = group_count.values().collect();
    vals.sort();
    vals.reverse();
    let mut count = 1;
    for i in 0..3 {
        count *= vals[i];
    }
    return count;
}

fn part_two(parsed: &Vec<(u128, u128, u128)>) -> u128 {
    let mut count = 1;
    let mut dist_heap: BinaryHeap<Reverse<(u128, (u128, u128, u128), (u128, u128, u128))>> =
        BinaryHeap::new();
    for i in 0..parsed.len() - 1 {
        for j in i + 1..parsed.len() {
            let dist = distance(parsed[i], parsed[j]);
            dist_heap.push(Reverse((dist, parsed[i], parsed[j])));
        }
    }
    let mut n = 0;
    let mut groups: HashMap<(u128, u128, u128), u128> = HashMap::new();
    let mut group_count: HashMap<u128, u128> = HashMap::new();
    let mut largest_count: u128 = 0;
    loop {
        if false {
            break;
        }
        let it = dist_heap.pop().unwrap();
        let gina = groups.contains_key(&it.0.1);
        let ginb = groups.contains_key(&it.0.2);
        if gina && !ginb {
            groups.insert(it.0.2, groups[&it.0.1]);
            group_count.insert(groups[&it.0.1], group_count[&groups[&it.0.1]] + 1);
            if group_count[&groups[&it.0.1]] > largest_count {
                largest_count = group_count[&groups[&it.0.1]];
            }
        }
        if !gina && ginb {
            groups.insert(it.0.1, groups[&it.0.2]);
            group_count.insert(groups[&it.0.2], group_count[&groups[&it.0.2]] + 1);
            if group_count[&groups[&it.0.2]] > largest_count {
                largest_count = group_count[&groups[&it.0.2]];
            }
        }
        if !gina && !ginb {
            groups.insert(it.0.1, n);
            groups.insert(it.0.2, n);
            group_count.insert(n, 2);
            n += 1;
        }
        if gina && ginb && groups[&it.0.1] != groups[&it.0.2] {
            let old_group = groups[&it.0.2];
            let new_group = groups[&it.0.1];
            let mut to_update: Vec<(u128, u128, u128)> = Vec::new();
            for node in groups.keys() {
                if old_group == groups[node] {
                    to_update.push(*node);
                    group_count.insert(new_group, group_count[&new_group] + 1);
                    group_count.insert(old_group, group_count[&old_group] - 1);
                }
            }
            for x in to_update {
                groups.insert(x, new_group);
            }

            if group_count[&new_group] > largest_count {
                largest_count = group_count[&new_group];
            }
        }
        if largest_count == parsed.len() as u128 {
            println!("{:?} {:?}", it.0.1, it.0.2);
            count = it.0.1.0 * it.0.2.0;
            break;
        }
    }
    return count;
}
