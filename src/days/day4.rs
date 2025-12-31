use std::error::Error;

use crate::utils::read_2d_chars;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Vec<Vec<char>> = read_2d_chars(input_path)?;
    
    let mut score: u64 = 0;
    for (pos_y, line) in lines.iter().enumerate() {
        for (pos_x, char) in line.iter().enumerate() {
            if char != &'@' {
                continue;
            }

            let mut nb_rolls = -1;
            
            let start_y = (pos_y as i32 - 1).max(0) as usize;
            let end_y = ((pos_y + 2) as usize).min(lines.len());

            let start_x = (pos_x as i32 - 1).max(0) as usize;
            let end_x = (pos_x + 2).min(line.len());

            for check_x in start_y..end_y {
                for check_y in start_x..end_x {
                    if lines[check_x][check_y] == '@' { 
                        nb_rolls += 1;
                    }
                }
            }

            if nb_rolls < 4 {
                score += 1;
            }
        }
    }

    return Ok(score);
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    // main loop, spiral inwards
    // when removing one roll, recursively check adjacent rolls.
    return Ok(0);
}