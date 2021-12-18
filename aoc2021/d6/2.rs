use std::io;
use std::io::prelude::*;

// To solve this problem I'll synchronize the fishes and advance
// a counter to see, in that particular day, how many fishes will generate
// offsprings. If so, the number of offsprings will be put in that 
fn main() {

    let mut reproducible : [u64; 7] = [0; 7]; // Fishes that are capable of reproducing
    let mut maturing : [u64; 2] = [0; 2]; // Fishes in maturing process

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);

    // Initial conditions
    let input : Vec<u8> = input.trim()
                               .split(',')
                               .map(|x| x.parse::<u8>().unwrap())
                               .collect();
    for fish in input {
        reproducible[fish as usize] += 1;
    }

    let mut generated_offsprings : u64 = 0;
    for day in 0..=256 {
        print!("day {} {:?} -- ", day % 7, reproducible);
        print!("day {} {:?}", day % 2, maturing);

        // Some logic inversion has led me to this. At this point
        // I don't fully understand why this happens...
        generated_offsprings = reproducible[day % 7];
        reproducible[day % 7] += maturing[day % 2];
        maturing[day % 2] = generated_offsprings;

        print!("-- generated: {}", generated_offsprings);
    
        let result : u64 = reproducible.iter().sum::<u64>() + maturing.iter().sum::<u64>();
        println!("-- number of lanternfish: {}", result);
    }


}
