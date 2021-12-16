
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut horizontal_pos : i32 = 0;
    let mut depth_pos : i32 = 0;
    let mut aim : i32 = 0;

    for line in lines {
        let input = line.expect("Line not good.");
        if input.len() < 1 {
            continue;
        }

        let tokens : Vec<&str> = input.split(" ").collect();

        let command = tokens[0];
        let value = tokens[1].parse::<i32>().expect("Invalid number");

        match command {
            "up" => { aim -= value },
            "down" => { aim += value },
            "forward" => { horizontal_pos += value; depth_pos += aim * value},
            _ => println!("Invalid command"),
        }

        println!("Command: {} -- Value: {}", command, value);
    }

    println!("Horizontal pos: {} Depth: {}", horizontal_pos, depth_pos);
    println!("result: {}", horizontal_pos * depth_pos);
}

