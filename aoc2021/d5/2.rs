use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Line {
    x_from : u16,
    y_from : u16,
    x_to : u16,
    y_to : u16
}

impl Line {
    fn is_horizontal(&self) -> bool {
        if self.x_from == self.x_to ||
           self.y_from == self.y_to {
               return true;
        }

        false
    }

    fn point_from_str(s: &str) -> (u16, u16) {
        let mut split = s.split(',');

        let x = split.next()
                     .unwrap()
                     .parse::<u16>()
                     .expect("Can't parse number!");

        let y = split.next()
                     .unwrap()
                     .parse::<u16>()
                     .expect("Can't parse number!");

        (x, y)
    }

    fn from_str(s: &str) -> Line {
        let mut split = s.split(' ');
        let mut ret = Line { x_from: 0,
                             y_from: 0,
                             x_to: 0,
                             y_to: 0,
                            };

        let from = split.next().unwrap();
        split.next();
        let to = split.next().unwrap();

        let (x, y) = Line::point_from_str(&from);

        ret.x_from = x;
        ret.y_from = y;

        let (x, y) = Line::point_from_str(&to);

        ret.x_to = x;
        ret.y_to = y;

        println!("Generated {:?}", ret);
        ret
    }
}

// Returns how many places have 2 lines overlapping
fn fill_board(board: &mut [u16; 1000 * 1000], line: &Line) -> u64 {
    let mut overlaps : u64 = 0;

    let mut i_rng = if line.y_from > line.y_to + 1 {
            (line.y_to..line.y_from+1)
        } else {
            (line.y_from..line.y_to+1)
        };
    
    for i in i_rng {
        let mut j_rng = if line.x_from > line.x_to + 1 {
                (line.x_to..line.x_from+1).into_iter()
            } else { 
                (line.x_from..line.x_to+1).into_iter()
            };
   
        for j in j_rng {
            let i : usize = i as usize;
            let j : usize = j as usize;

            board[i * 1000 + j] += 1;
            if board[i * 1000 + j] == 2 {
                overlaps += 1;
            }
        }
    }

    overlaps
}

// Similar logic, but this one iterates simultaneously over the dimensions.
// I know it's bad to use collections, but i found no other way to reverse a
// range and it's been a couple of hours already.
fn fill_board_diagonal(board: &mut [u16; 1000 * 1000], line: &Line) -> u64 {
    let mut overlaps : u64 = 0;

    let mut i_rng : Vec<u16> = if line.y_from > line.y_to + 1 {
            (line.y_to..line.y_from+1).rev().collect()
        } else {
            (line.y_from..line.y_to+1).collect()
        };
 
    let mut j_rng : Vec<u16> = if line.x_from > line.x_to + 1 {
            (line.x_to..line.x_from+1).rev().collect()
        } else { 
            (line.x_from..line.x_to+1).collect()
        };

    let mut j_it = j_rng.iter();

    for i in i_rng {
            let i : usize = i as usize;
            let j : usize = j_it.next().unwrap().clone() as usize;

            board[i * 1000 + j] += 1;
            if board[i * 1000 + j] == 2 {
                overlaps += 1;
            }
    }

    overlaps
}

fn main() {
    let stdin = io::stdin();
    let entries = stdin.lock().lines();

    // Board 1000 * 1000
    let mut board : [u16; 1000 * 1000] = [0; 1000 * 1000];
    let mut overlaps : u64 = 0;

    for entry in entries {
        // Expected line format:
        // 0,9 -> 5,9
        // println!("Got {:?}", line);
        let entry = entry.unwrap();

        if entry.len() == 0 {
            continue;
        }

        let line = Line::from_str(&entry);

        if line.is_horizontal() {
            // Fill in the board
            overlaps += fill_board(&mut board, &line);
        } else {
            overlaps += fill_board_diagonal(&mut board, &line);
        }
    }

    println!("Result: {}", overlaps);
}
