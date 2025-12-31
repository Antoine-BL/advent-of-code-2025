mod days;
pub mod utils;

use std::{env, error::Error, num::ParseIntError};

const SOLUTIONS: [(& dyn Fn(&str)->Result<u64, Box<dyn Error>>, & dyn Fn(&str)->Result<u64, Box<dyn Error>>); 12] = [
    (&days::day1::part1,  &days::day1::part2),
    (&days::day2::part1,  &days::day2::part2),
    (&days::day3::part1,  &days::day3::part2),
    (&days::day4::part1,  &days::day4::part2),
    (&days::day5::part1,  &days::day5::part2),
    (&days::day6::part1,  &days::day6::part2),
    (&days::day7::part1,  &days::day7::part2),
    (&days::day8::part1,  &days::day8::part2),
    (&days::day9::part1,  &days::day9::part2),
    (&days::day10::part1, &days::day10::part2),
    (&days::day11::part1, &days::day11::part2),
    (&days::day12::part1, &days::day12::part2),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print!("Not enough args");
        return;
    }
    
    let (day, part, is_test) = parse_args(&args).unwrap();

    let test_suffix = if is_test {
        "_test"
    } else {
        ""
    };
    let input_path = format!("./src/inputs/day{}{}", day, test_suffix);
    
    let (day_pt1, day_pt2) = SOLUTIONS.get((day - 1) as usize).unwrap();
    
    let start = std::time::SystemTime::now();
    let result = if part == 1 {
        day_pt1(input_path.as_str())
    } else {
        day_pt2(input_path.as_str())
    };
    match result {
        Ok(solution) => {
            println!("Solution: {}", solution);
            let elapsed = start.elapsed().unwrap();
            println!("runtime: {}us", elapsed.as_micros());
        }
        Err(err) => println!("{}", err)
    }
    

}

fn parse_args(args: &Vec<String>) -> Result<(i8, i8, bool), ParseIntError> {
    let day: i8 = args[1].parse::<i8>()?;
    let part: i8 = args[2].parse::<i8>()?;
    let is_test = match args.get(3) {
        Some(val) => val == "true",
        None => false
    };

    return Ok((day, part, is_test));
}