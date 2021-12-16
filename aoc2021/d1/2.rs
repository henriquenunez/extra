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
    let mut count = 0;

    let mut window : [i32; 3] = [0; 3];
    let mut cursor = 0; // To place inside the window
    let mut curr_sum = 0;
    let mut last_sum = 300000;
    let mut first_buff = 3;

    for val in values {
        curr_sum -= window[cursor];
        window[cursor] = val as i32;
        curr_sum += window[cursor];

        cursor = (cursor + 1) % 3;

        if first_buff > 0 {
            first_buff -= 1;
            continue;
        }

        if curr_sum > last_sum {
            println!("Increased: {} -> {}", last_sum, curr_sum);
            count += 1;
        }

        last_sum = curr_sum;
    }

    println!("Total increases: {}", count);
}

