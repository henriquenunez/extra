use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

// Infers the segments of the 7 segment display
// given the full list of digits printed. The explanation
// for this algorithm can be found in the picture
// in this very directory.
fn parse_digits(l: &str, t: &str) -> u64 {

    let one = l.split(" ")
               .filter(|z| z.len() == 2)
               .next()
               .unwrap()
               .chars()
               .collect::<HashSet<char>>();

    let four = l.split(" ")
                .filter(|z| z.len() == 4)
                .next()
                .unwrap()
                .chars()
                .collect::<HashSet<char>>();

    let seven = l.split(" ")
                 .filter(|z| z.len() == 3)
                 .next()
                 .unwrap()
                 .chars()
                 .collect::<HashSet<char>>();

    let eight = l.split(" ")
                 .filter(|z| z.len() == 7)
                 .next()
                 .unwrap()
                 .chars()
                 .collect::<HashSet<char>>();

    let alpha = seven.difference(&one)
                     .cloned()
                     .collect::<HashSet<char>>();

    let beta_delta = four.difference(&one)
                         .cloned()
                         .collect::<HashSet<char>>();
    
    let epsilon_eta = eight.difference(&four.union(&alpha)
                                            .cloned()
                                            .collect::<HashSet<char>>())
                           .cloned()
                           .collect::<HashSet<char>>();
   
    let three = l.split(" ")
                 .filter(|z| z.len() == 5)
                 .filter(|z| z.chars().collect::<HashSet<char>>().is_superset(&one))
                 .next()
                 .unwrap()
                 .chars()
                 .collect::<HashSet<char>>();

    let beta = eight.difference(&three)
                    .cloned()
                    .collect::<HashSet<char>>()
                    .intersection(&beta_delta)
                    .cloned()
                    .collect::<HashSet<char>>();

    let epsilon = eight.difference(&three)
                       .cloned()
                       .collect::<HashSet<char>>()
                       .intersection(&epsilon_eta)
                       .cloned()
                       .collect::<HashSet<char>>();

    let two = l.split(" ")
               .filter(|z| z.len() == 5)
               .filter(|z| z.chars().collect::<HashSet<char>>().is_superset(&epsilon))
               .next()
               .unwrap()
               .chars()
               .collect::<HashSet<char>>();

    let five = l.split(" ")
                .filter(|z| z.len() == 5)
                .filter(|z| z.chars().collect::<HashSet<char>>().is_superset(&beta))
                .next()
                .unwrap()
                .chars()
                .collect::<HashSet<char>>();

    let six = l.split(" ")
               .filter(|z| z.len() == 6)
               .filter(|z| !z.chars().collect::<HashSet<char>>().is_superset(&one))
               .next()
               .unwrap()
               .chars()
               .collect::<HashSet<char>>();

    let zero = l.split(" ")
                .filter(|z| z.len() == 6)
                .filter(|z| !z.chars().collect::<HashSet<char>>().is_superset(&beta_delta))
                .next()
                .unwrap()
                .chars()
                .collect::<HashSet<char>>();

    let nine = eight.difference(&epsilon)
                    .cloned()
                    .collect::<HashSet<char>>();

    // Just sanity checking
    println!("0: {:?}", zero);
    println!("1: {:?}", one);
    println!("2: {:?}", two);
    println!("3: {:?}", three);
    println!("4: {:?}", four);
    println!("5: {:?}", five);
    println!("6: {:?}", six);
    println!("7: {:?}", seven);
    println!("8: {:?}", eight);
    println!("9: {:?}", nine);

    println!("epsilon: {:?}", epsilon);
    println!("beta: {:?}", beta);
    println!("alpha: {:?}", alpha);

    let mut checksum : u64 = 0;

    for digit in t.split(" ") {
        if digit.len() == 0 {
            continue;
        }

        checksum *= 10;
        checksum += match digit.chars().collect::<HashSet<char>>() {
            x if x == zero => 0,
            x if x == one => 1,
            x if x == two => 2,
            x if x == three => 3,
            x if x == four => 4,
            x if x == five => 5,
            x if x == six => 6,
            x if x == seven => 7,
            x if x == eight => 8,
            x if x == nine => 9,
            _ => {panic!("Fatal error while parsing {}", digit)},
        };

        println!("{}: {}", digit, checksum);
    }
   
    checksum
}


fn process_line(l: &str) -> u64 {
    let mut parts = l.split("|");

    parse_digits(parts.next().unwrap(), parts.next().unwrap())
    //println!("{}", parts.next().unwrap());
    //println!("after {}",);

    //let after = parts.next().unwrap();
    //compute_1478(&after)
}

fn main() {
    let stdin = io::stdin();
    let mut count : u64 = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Error acquiring line!");

        if line.len() == 0 {
            continue;
        }

        count += process_line(&line);
    }

    println!("Result {}", count);
}
