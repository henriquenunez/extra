use std::io;
use std::io::prelude::*;

// Since the digits 1 4 7 8 have a unique amount of
// segments, we can easily compute this
fn compute_1478(l: &str) -> u16 {
    let mut count : u16 = 0;

    for digit in l.split(" ") {
        if digit.len() == 0 {
            continue;
        }

        // 1 -> 2 segments
        // 4 -> 4 segments
        // 7 -> 3 segments
        // 8 -> 7 segments
        match digit.len() {
            2 | 3 | 4 | 7 => count += 1,
            _ => {},
        }

        println!("{}", digit);
    }
    count
}

fn process_line(l: &str) -> u16 {
    let mut parts = l.split("|");

    parts.next();
    //println!("{}", parts.next().unwrap());
    //println!("after {}",);

    let after = parts.next().unwrap();
    compute_1478(&after)
}

fn main() {
    let stdin = io::stdin();
    let mut count : u16 = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Error acquiring line!");

        if line.len() == 0 {
            continue;
        }

        count += process_line(&line);
    }

    println!("Result {}", count);
}
