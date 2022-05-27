/*
 * second prob was just a refinement of the first
 **/

use regex::Regex;
use std::fs;

fn main() {
    let sol = problem();
    println!("sol: {}", sol)
}

fn get_input() -> String {
    fs::read_to_string("./input").expect("something went wrong while reading input")
}

#[derive(Default, Debug)]
struct Position {
    depth: i32,
    forward: i32,
    aim: i32,
}

impl Position {
    fn up(&mut self, value: i32) {
        self.aim -= value;
    }

    fn down(&mut self, value: i32) {
        self.aim += value;
    }

    fn forward(&mut self, value: i32) {
        self.forward += value;
        self.depth += self.aim * value;
    }
}

fn get_value(line: &str) -> i32 {
    let split: Vec<&str> = line.split(' ').collect();
    let value: i32 = split[1].parse().unwrap();
    value
}
fn problem() -> i32 {
    let forward_re = Regex::new(r"forward ").unwrap();
    let up_re = Regex::new(r"up ").unwrap();
    let down_re = Regex::new(r"down ").unwrap();

    // i32s default to 0, exactly what we need
    let mut position: Position = Default::default();

    for line in get_input().lines() {
        if forward_re.is_match(line) {
            position.forward(get_value(line));
        } else if up_re.is_match(line) {
            position.up(get_value(line));
        } else if down_re.is_match(line) {
            position.down(get_value(line));
        }
    }

    position.depth * position.forward
}
