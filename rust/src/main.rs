use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    run_days();
}

fn run_days() {
    let utils::DayResult(part_1, part_2) = day9::run(get_input_path(9));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("9-1: {} | {}", res_1, time_1);
    println!("9-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day8::run(get_input_path(8));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("8-1: {} | {}", res_1, time_1);
    println!("8-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day7::run(get_input_path(7));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("7-1: {} | {}", res_1, time_1);
    println!("7-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day6::run(get_input_path(6));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("6-1: {} | {}", res_1, time_1);
    println!("6-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day5::run(get_input_path(5));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("5-1: {} | {}", res_1, time_1);
    println!("5-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day4::run(get_input_path(4));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("4-1: {} | {}", res_1, time_1);
    println!("4-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day3::run(get_input_path(3));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("3-1: {} | {}", res_1, time_1);
    println!("3-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day2::run(get_input_path(2));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("2-1: {} | {}", res_1, time_1);
    println!("2-2: {} | {}", res_2, time_2);
    let utils::DayResult(part_1, part_2) = day1::run(get_input_path(1));
    let utils::PartResult(res_1, time_1) = part_1;
    let utils::PartResult(res_2, time_2) = part_2;
    println!("1-1: {} | {}", res_1, time_1);
    println!("1-2: {} | {}", res_2, time_2);
}

fn get_input_path(day: u32) -> String {
    let args: Vec<String> = env::args().collect();
    let file_name = format!("input-{}.txt", day);
    if args.len() < 2 {
        return "/".to_owned() + &file_name;
    }
    let last_char = args[1].chars().last();
    return format!(
        "{}{}",
        args[1],
        if last_char.unwrap() == '/' {
            file_name
        } else {
            "/".to_owned() + &file_name
        }
    );
}
