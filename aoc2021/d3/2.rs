
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
   
    let mut first = true;
    let mut count : u32 = 0;

    // Transform into numbers
    let mut numbers : Vec<u32> = Vec::new();
    //let num_len = lines.next().unwrap().unwrap().len();

    let mut n_len : u32 = 0;

    for line in lines {
        let line_ = line.unwrap();
        let curr_line_ref = line_.as_str();
 
        if curr_line_ref.len() == 0 {
            break;
        }
       
        n_len = curr_line_ref.len() as u32;

        let val : u32 = u32::from_str_radix(curr_line_ref, 2).unwrap();

        //lines.map(|x| u32::from_str_radix(x.unwrap().as_str(), 2).unwrap())
        //                          .collect();

        //println!("Got {}", val);
        numbers.push(val);

        count += 1;
    }

    /*
    for num in numbers.iter() {
        println!("{}", num);
    }
    */

    let mut mask : u32 = 0x1 << (n_len - 1);

    // [1 2 3 4 5] (x > 4) -> [5]
    // [2 3 4 5 6] (x : x + 1)

    let mut oxygen_v : Vec<u32> = numbers.iter().cloned().collect();
    let mut co2_v: Vec<u32> = numbers.iter().cloned().collect();

    for i in 0..n_len {

        // Find the most common in each array
        let amount_bits_o2 = oxygen_v.iter().filter(|x| *x & mask > 0).count() as u32;
        let amount_nulls_o2: u32 = oxygen_v.len() as u32 - amount_bits_o2;

        let amount_bits_co2 = co2_v.iter().filter(|x| *x & mask > 0).count() as u32;
        let amount_nulls_co2 : u32 = co2_v.len() as u32 - amount_bits_co2;

        if oxygen_v.len() > 1 {
            if amount_bits_o2 >= amount_nulls_o2 { // Most common is 1
                oxygen_v = oxygen_v.iter().filter(|x| *x & mask > 0).cloned().collect(); // Filter with mask 1
            } else {
                oxygen_v = oxygen_v.iter().filter(|x| *x & mask == 0).cloned().collect();
            }
        }

        if co2_v.len() > 1 {
            if amount_bits_co2 >= amount_nulls_co2 { // Most common is 1
                co2_v = co2_v.iter().filter(|x| *x & mask == 0).cloned().collect(); // Filter with mask 0
            } else {
                co2_v = co2_v.iter().filter(|x| *x & mask > 0).cloned().collect();
            }
        }

        mask = mask >> 1;
    }

    for num in oxygen_v.iter() {
        println!("Ox: {}", num);
    }

    for num in co2_v.iter() {
        println!("Co2: {}", num);
    }

    println!("Result: {}", oxygen_v[0] * co2_v[0]);
}

