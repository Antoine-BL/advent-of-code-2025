mod days;
pub mod utils;

use std::{env, error::Error, num::ParseIntError};

const SOLUTIONS: [(& dyn Fn(&str)->Result<(), Box<dyn Error>>, & dyn Fn(&str)->Result<(), Box<dyn Error>>); 1] = [
    (&days::day1::part1, &days::day1::part2)
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
        Ok(_) => {
            let elapsed = start.elapsed().unwrap();
            println!("{}ms", elapsed.as_millis());
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