use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_2d_chars<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where P: AsRef<Path> {
    let contents: String = fs::read_to_string(filename)?;
    let data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    return Ok(data);
}