use std::io;

// Thanks to euler back in the 18th century we know
// how to compute arithmetic progressions.
fn compute_fuel(crabs: &Vec<u16>, pos: i32) -> i32 {
    crabs.iter()
         .map(|x| (*x as i32 - pos).abs() * (1 + (*x as i32 - pos).abs()) / 2)
         .sum()
}

// Given an array of horizontal positions, return
// the sum of the steps the elements must take so
// all of them can be in a single position. This single
// position is the one that will yeld the lowest number
// of steps.
fn main() {
    let mut crabs = String::new();
    io::stdin().read_line(&mut crabs);
    let crabs : Vec<u16> = crabs.trim()
                                .split(',')
                                .map(|x| x.parse::<u16>().unwrap())
                                .collect();

    //let crab_amount = crabs.iter().count();
    //let best_position : u16 = (crabs.iter().sum::<u16>() as f32 / crab_amount as f32).round() as u16;

    let max = *crabs.iter().max().unwrap();
    let lowest_fuel_cost = (0..=max).map(|z| compute_fuel(&crabs, z as i32)).min().unwrap();

    println!("Total fuel {}", lowest_fuel_cost);
}
