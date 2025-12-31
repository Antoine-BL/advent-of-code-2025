use std::error::Error;
use std::ops::Add;
use std::{fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;
    let mut lines_ok = lines.map_while(Result::ok);

    let mut tiles: Vec<(u64, u64)> = Vec::new();
    let first_tile = parse_tile(lines_ok.next().unwrap().as_str())?;
    tiles.push(first_tile);
    let mut largest_area = 0;

    let mut tile_pair = ((0,0), (0,0));
    for line in lines_ok {
        let cur_tile = parse_tile(line.as_str())?;
        for tile in &tiles {
            let cur_area = calc_area(cur_tile, *tile);
            if cur_area > largest_area {
                tile_pair = (cur_tile, *tile);
                largest_area = cur_area;
            }
        }
        tiles.push(cur_tile);
    }

    let ((x1, y1), (x2, y2)) = tile_pair;
    println!("{},{} {},{}", x1, y1, x2, y2);
    return Ok(largest_area as u64);
}

fn parse_tile(line: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let parsed: Vec<u64> = line.split(",").map(|x| x.parse()).map_while(Result::ok).collect();
    Ok((parsed[0], parsed[1]))
}

fn calc_area(first_tile: (u64, u64), second_tile: (u64, u64)) -> u64 {
    let (x1, y1) = first_tile;
    let (x2, y2) = second_tile;

    return x1.abs_diff(x2).add(1) * y1.abs_diff(y2).add(1);
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}