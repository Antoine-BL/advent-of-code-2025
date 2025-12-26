use std::{error::Error, fs::File, io::{self, BufReader}, ops::Div};
use crate::utils::read_lines;

const START_POS: i32 = 50;
const MAX_POS: i32 = 100;

pub fn part1(input_path: &str) -> Result<(), Box<dyn Error>> {
    let lines: io::Lines<BufReader<File>> = read_lines(input_path)?;
   
    let mut score = 0;
    let mut pos = START_POS;
    for line in lines.map_while(Result::ok) {
        let mut chars = line.chars();
        let dir_mod = match chars.next() {
            Some(dir) => if dir == 'L' {
                -1
            } else {
                1
            },
            _ => return Err(format!("invalid instruction {}", line).into())
        };
        let n_turns = chars.as_str().parse::<i32>()?;

        pos += n_turns * dir_mod;
        pos = pos % MAX_POS;
        
        if pos == 0 {
            score += 1;
        }
    }

    println!("{}", score);

    return Ok(())
}

pub fn part2(input_path: &str) -> Result<(), Box<dyn Error>> {
    let lines: io::Lines<BufReader<File>> = read_lines(input_path)?;
   
    let mut score = 0;
    let mut pos = START_POS;
    for line in lines.map_while(Result::ok) {
        let mut chars = line.chars();
        let dir_mod = match chars.next() {
            Some(dir) => if dir == 'L' {
                -1
            } else {
                1
            },
            _ => return Err(format!("invalid instruction {}", line).into())
        };
        let n_turns = chars.as_str().parse::<i32>()?;

        let prev_pos = pos;
        pos += n_turns * dir_mod;
        
        score += (pos / MAX_POS).abs();

        
        if pos <= 0 {
            score += 1;
        } else if (prev_pos > 0 && pos < 0) || (prev_pos < 0 && pos > 0) {
            score += 1;
        }

        pos = pos % MAX_POS;
    }

    println!("{}", score);

    return Ok(())
}