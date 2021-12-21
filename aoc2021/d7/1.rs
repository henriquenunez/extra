use std::io;

fn compute_fuel(crabs: &Vec<u16>, pos: i32) -> i32 {
    crabs.iter()
         .map(|x| (*x as i32 - pos).abs())
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

    let max = *crabs
                .iter()
                .max()
                .unwrap();

    let lowest_fuel_cost = (0..=max)
                                .map(|z| compute_fuel(&crabs, z as i32))
                                .min()
                                .unwrap();

    println!("Total fuel {}", lowest_fuel_cost);
}
