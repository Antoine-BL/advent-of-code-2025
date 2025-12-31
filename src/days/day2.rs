use std::{error::Error, fs::File, io::{BufReader, Lines}};
use crate::utils::read_lines;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;

    let mut score = 0;
    for line in lines.map_while(Result::ok) {
        for range in line.split(',') {
            score += validate_range(range)?;
        }
    }

    println!("{}", score);

    return Ok(0);
}

fn validate_range(range: &str) -> Result<u64, Box<dyn Error>> {
    let mut score: u64 = 0;

    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() < 2 {
        return Ok(0);
    }
    let start: u64 = parts[0].parse::<u64>()?;
    let end: u64 = parts[1].parse::<u64>()? + 1;
    for id in start..end {
        if !validate_id(format!("{}", id).as_str())? {
            println!("id: {}", id);
            score += id;
        }
    }

    return Ok(score);
}

fn validate_id(id: &str) -> Result<bool, Box<dyn Error>> {
    let length = id.chars().count();
    if length % 2 == 1 {
        return Ok(true);
    };

    let (l_half, r_half) = id.split_at(length / 2);

    return Ok(l_half != r_half)
}

pub fn part2(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;

    let mut score = 0;
    for line in lines.map_while(Result::ok) {
        for range in line.split(',') {
            score += validate_range2(range)?;
        }
    }

    println!("{}", score);

    return Ok(0);
}

fn validate_range2(range: &str) -> Result<u64, Box<dyn Error>> {
    let mut score: u64 = 0;

    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() < 2 {
        return Ok(0);
    }
    let start: u64 = parts[0].parse::<u64>()?;
    let end: u64 = parts[1].parse::<u64>()? + 1;
    for id in start..end {
        if !validate_id2(format!("{}", id).as_str())? {
            println!("id: {}", id);
            score += id;
        }
    }

    return Ok(score);
}

fn validate_id2(id: &str) -> Result<bool, Box<dyn Error>> {
    let length = id.chars().count();
    let (l_half, r_half) = id.split_at(length / 2);

    return Ok(l_half != r_half)
}