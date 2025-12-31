use std::{error::Error, fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;


pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;

    let mut score: u64 = 0;
    for line in  lines.map_while(Result::ok) {
        score += optimize_battery_bank(line.as_str());
    }
    
    return Ok(score);
}

fn optimize_battery_bank(bank: &str) -> u64 {
    let mut first = 0;
    let mut second = 0;

    let mut chars = bank.chars();
    let mut prev = chars.next().unwrap().to_digit(10).unwrap() as u64;
    for char in chars {
        let cur: u64 = char.to_digit(10).unwrap() as u64;

        if prev > first {
            first = prev;
            second = 0;
        }

        if cur > second {
            second = cur;
        }
        
        prev = cur;
    }

    println!("{}{}", first, second);
    return first * 10 + second;
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}