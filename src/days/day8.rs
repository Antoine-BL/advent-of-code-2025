use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;
use std::{fs::File, io::{BufReader, Lines}};
use ordered_float::OrderedFloat;

use crate::utils::read_lines;

use std::cmp::{Ordering, Ord, PartialOrd, PartialEq};

type Point = (u64, u64, u64);

const NB_CONNECTIONS: usize = 1000;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Connection {
    pub start: Point,
    pub end: Point,
    pub dist: OrderedFloat<f64>,
}

impl Connection {
    fn new(start: Point, end: Point) -> Self {
        Self {
            start: start,
            end: end,
            dist: calc_distance(&start, &end)
        }
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines<BufReader<File>> = read_lines(input_path)?;

    let boxes: Vec<Point> = lines.map_while(Result::ok).map(|x| parse_jn_box(x.as_str())).collect();
    let mut connections: [Option<Connection>; NB_CONNECTIONS] = [None; NB_CONNECTIONS];
    // Determine actual connections
    for x in 0..boxes.len() - 1 {
        let new_box = boxes[x]; 
        for y in x + 1..boxes.len() {
            let other_box = boxes[y];
            let conn = Connection::new(new_box, other_box);

            for i in 0..connections.len() {
                let queue_item: Option<Connection> = connections[i];

                if queue_item.is_some() {
                    let other_conn = queue_item.unwrap();
                    if conn.dist < other_conn.dist && i < connections.len() - 1{
                        let mut value = connections[i];
                        connections[i] = Some(conn);
                        for j in i..connections.len() - 1 {
                            let temp = connections[j + 1];
                            connections[j + 1] = value;
                            value = temp;

                            if value.is_none() {
                                break;
                            }
                        }
                        break;
                    }
                } else {
                    connections[i] = Some(conn);
                    break;
                }
            }
        }
    }

    // Build graph
    let mut edges: HashMap<Point, Vec<Point>> = HashMap::with_capacity(NB_CONNECTIONS);
    for conn in connections.iter().filter(|x| x.is_some()).map(|x| x.unwrap()) {
        match edges.get_mut(&conn.start) {
            Some(connected) => connected.push(conn.end),
            None => {
                edges.insert(conn.start, vec![conn.end]);
            }
        }
        match edges.get_mut(&conn.end) {
            Some(connected) => connected.push(conn.start),
            None => {
                edges.insert(conn.end, vec![conn.start]);
            }
        }
    }

    let mut sizes: BinaryHeap<u64> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    for cur_box in boxes {
        if visited.contains(&cur_box) {
            continue;
        }
        visited.insert(cur_box);
        let size = walk(&cur_box, &edges, &mut visited);
        sizes.push(size);
    }

    let score = sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap();

    Ok(score)
}

fn walk(cur_pos: &Point, edges: &HashMap<Point, Vec<Point>>, visited: &mut HashSet<Point>) -> u64 {
    let mut size = 1;
    match edges.get(cur_pos) {
        Some(conn_points) => {
            for pt in conn_points {
                if !visited.contains(pt) {
                    visited.insert(*pt);
                    size += walk(pt, edges, visited);
                }
            }
            size
        },
        None => size
    }
}

fn calc_distance(p1: &(u64, u64, u64), p2: &(u64, u64, u64)) -> OrderedFloat<f64> {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;

    let dist = ((x1.abs_diff(*x2).pow(2) + y1.abs_diff(*y2).pow(2) + z1.abs_diff(*z2).pow(2)) as f64).sqrt();

    OrderedFloat::from(dist)
}

fn parse_jn_box(line: &str) -> (u64, u64, u64) {
    let coords: Vec<u64> = line.split(",").take(3).map_while(Option::Some).map(str::parse).map_while(Result::ok).collect();
    (coords[0], coords[1], coords[2])
}

pub fn part2(_input_path: &str) -> Result<u64, Box<dyn Error>> {
    Ok(0)
}