use std::error::Error;

use crate::utils::read_2d_chars;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut data: Vec<Vec<char>> = read_2d_chars(input_path)?;

    let mut score: u64 = 0;
    for i in 1..data.len() {
        let line_len = data[i].len();
        for j in 0..line_len {
            if data[i - 1][j] == '.' || data[i-1][j] == '^' {
                continue;
            }

            let line = &mut data[i];
            if line[j] == '^' {
                score += 1;
                if j > 0 {
                    line[j - 1] = '|';
                }
                if j < line.len() - 1 {
                    line[j + 1] = '|';
                }
            } else {
                line[j] = '|';
            }
        }   
    }

    return Ok(score);
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}