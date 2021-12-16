
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
   
    let mut buffers : Vec<u16> = Vec::new();
    let mut first = true;
    let mut count : u32 = 0;

    for line in lines {
        let line_ = line.unwrap();
        let curr_line_ref = line_.as_str();
    
        println!("Got {}", line_);

        if curr_line_ref.len() == 0 {
            break;
        }
       
        count += 1;

        if first {
            println!("Creating vector with capacity: {}", curr_line_ref.len());
            buffers = vec![0; curr_line_ref.len()];
            first = false;
        }

        // Do operations with the line
        for (idx, bin) in curr_line_ref.chars().enumerate() {
            buffers[idx] += (bin as u8 - 48) as u16;
        }
    }

    let thresh : u32 = count / 2;
    let mut gamma : u32 = 0;

    for (mag, &item) in buffers.iter().rev().enumerate() {
        print!("{}.", item);

        let m = match item as u32 > thresh { // If the item is larger than count, 
            true => 1,
            _ => 0,
        };

        gamma += m as u32 * 2_u32.pow(mag as u32);
    }
    print!("\n");

    // buffers.len is the numbers of bits in the original number.
    // Raising 2 to this number and subtracting 1 yields the maximum unsigned number.
    // Subracting gamma to it will give the inverted result, since if all bits are set
    // it's equal to XOR.
    let epsilon = 2_u32.pow(buffers.len() as u32) - 1 - gamma;
    println!("Gamma: {:b} -- Epsilon: {:b} -- Result: {}", gamma, epsilon, gamma*epsilon);
}

