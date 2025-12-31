use std::collections::HashMap;
use std::error::Error;
use std::{fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;

const START: &str = "you";

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;
    let mut outputs: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in lines.map_while(Result::ok) {
        let mut split = line.split_whitespace();
        let src: &str = {
            let s = split.next().unwrap();
            &s[0..s.len() - 1]
        };
        let dsts: Vec<String> = split.map(String::from).collect();
        
        outputs.insert(String::from(src), dsts);
    }

    let score = walk(START, &outputs);

    return Ok(score);
}

fn walk(cur_output: &str, outputs: &HashMap<String, Vec<String>>) -> u64 {
    if cur_output == "out" {
        1
    } else {
        let mut score: u64 = 0;
        for output in outputs[cur_output].iter() {
            score += walk(output, outputs);
        }
        score
    }
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}