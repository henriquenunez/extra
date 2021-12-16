
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut curr_line_ref : String;

    if let Some(Ok(line)) = lines.next() {
        curr_line_ref = line;
    } else { return };
    
    let mut buffers : Vec<u16> = vec![0; curr_line_ref.len()];

    loop {
        // Do operations with the line
        for (idx, bin) in curr_line_ref.chars().enumerate() {
            buffers[idx] += (bin as u8 - 48) as u16;
        }

        if let Some(Ok(line)) = lines.next() {
            curr_line_ref = line;
        } else { break };

        if curr_line_ref.len() == 0 {
            break;
        }
    }

    for &item in buffers.iter() {
        print!("{}.", item);
    }

    print!("\n");
}

