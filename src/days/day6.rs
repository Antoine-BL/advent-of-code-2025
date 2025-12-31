use std::error::Error;
use std::{fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let reader: Lines<BufReader<File>> = read_lines(input_path)?;
    let lines: Vec<String> = reader.map_while(Result::ok).collect();

    let operators: Vec<&str> = lines.last().unwrap().split_whitespace().collect();
    let mut results: Vec<u64> = vec![0; operators.len()];
    for (i, op) in operators.iter().enumerate() {
        results[i] = if *op == "*" {
            1
        } else {
            0
        };
    }

    for line in lines.iter().take(lines.len() - 1) {
        let operands = line.split_whitespace();
        
        for (i, operand) in operands.enumerate() {
            let parsed: u64 = operand.parse()?;
            let op = operators[i];
            if op == "*" {
                results[i] *= parsed;
            } else {
                results[i] += parsed;
            }
        }
    }

    let result = results.iter().sum();
    
    return Ok(result);
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}