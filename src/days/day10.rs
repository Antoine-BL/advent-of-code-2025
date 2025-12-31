use std::collections::HashSet;
use std::error::Error;
use std::{fs::File, io::{BufReader, Lines}};

use crate::utils::read_lines;

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;

    let mut answer = 0;
    for line in lines.map_while(Result::ok) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let lights = parse_lights(parts[0]);
        let buttons_str = parts.iter().skip(1).take(parts.len() - 2).map(|x| *x).collect();
        let buttons = parse_buttons(buttons_str, parts[0].len() - 2);

        answer += test_buttons(lights, buttons);
    }

    return Ok(answer);
}

fn parse_lights(lights: &str) -> u32 {
    let n_lights = lights.len() - 2;
    lights.chars()
        .skip(1).take(n_lights)
        .enumerate()
        .map(|(i, x)| {
            if x == '#' {
                (1 as u32) << (n_lights - (i + 1))
            } else {
                0 as u32
            }
        })
        .sum::<u32>()
}

fn parse_buttons(buttons: Vec<&str>, n_lights: usize) -> Vec<u32> {
    buttons.into_iter().map(|b| {
        b.strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .split(",")
            .map(|x| {
                let digit = x.parse::<u32>().unwrap();
                1 << (n_lights as u32 - digit - 1 as u32)
            })
            .sum()
    }).collect()
}

fn test_buttons(lights: u32, btns: Vec<u32>) -> u64 {
    let mut nb_presses = 0;
    let mut states = vec![0];
    let mut next_states: Vec<u32> = Vec::new();
    let mut tested_states: HashSet<u32> = HashSet::new();
    tested_states.insert(0);
    loop {
        nb_presses += 1;
        let mut found = false;
        for btn in btns.iter() {
            for state in states.iter() {
                let new_state = state ^ btn;
                
                found = new_state == lights;
                if found {
                    break;
                }
                
                if !tested_states.contains(&new_state) {
                    next_states.push(new_state);
                    tested_states.insert(new_state);
                }
            }

            if found {
                break;
            }
        }
        states.clear();
        states.append(&mut next_states);
        if found {
            break;
        }
    }

    nb_presses
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    return Ok(0);
}
