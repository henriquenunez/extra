use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();

    let values : Vec<u32> = input.filter(|x| x.is_ok())
                                 .map(|x| x.unwrap())
                                 .filter(|x| x.len() > 0)
                                 .filter_map(|x| x.parse::<u32>().ok())
                                 .collect();

    let mut last_val : i32 = 30000;
    let mut curr_val : i32;
    let mut count = 0;

    for val in values {
        curr_val = val as i32;

        if curr_val > last_val {
            println!("Increased: {} -> {}", last_val, curr_val);
            count += 1;
        }

        last_val = curr_val;
    }

    println!("Total increases: {}", count);
}

