use std::error::Error;
use std::{fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut ok_lines = lines.map_while(Result::ok);
    let mut ok_iter = ok_lines.by_ref();
    for line in &mut ok_iter {
        if line == "" {
            break;
        }

        let mut splitted = line.split("-");
        let low = splitted.next().unwrap().parse::<u64>()?;
        let high = splitted.next().unwrap().parse::<u64>()?;

        ranges.push((low, high));
    }

    let mut score = 0;
    for line in &mut ok_iter {
        let parsed = line.parse::<u64>()?;

        for (low, high) in &ranges {
            if parsed >= *low && parsed <= *high {
                score += 1;
                break;
            }
        }
    }

    return Ok(score);
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}