use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct BingoCell {
    row : u8,
    col : u8,
    valid : bool,
}

struct BingoBoard {
    cells : [BingoCell; 100], // Cells referenced by value
    score : u16,
    rows : [u8; 5], // To store how many values have been marked
    cols : [u8; 5], // in the rows or columns
    won : bool,
    last_num_called : u8,
    times_to_win : u16, //How many marks it took for the board to win
}

impl BingoBoard {
    pub fn mark(&mut self, value : u8) {

        let value : usize = value as usize;

        if !self.cells[value].valid {
            return;
        }

        self.cells[value].valid = false;

        let BingoCell { row, col, .. } = self.cells[value];

        let row : usize = row as usize;
        let col : usize = col as usize;

        self.rows[row] += 1;
        self.cols[col] += 1;

        //println!("Marking val {} {}: {}", row, col, value);

        self.score -= value as u16;
        self.last_num_called = value as u8;

        self.won = self.rows[row] == 5 || self.cols[col] == 5;
    }

    pub fn apply_marks(&mut self, marks: &String) {
        for split in marks.split(',') {
            self.times_to_win += 1;
            //println!("spl {}", split);
            if self.won {
                println!("WON");
                return;
            }
            self.mark(split.parse::<u8>().expect("Unable to parse"));
        }
    }

    pub fn init(board: &String) -> BingoBoard {
        let mut ret = BingoBoard { cells: [BingoCell {row: 0, col: 0, valid: false}; 100],
                               score: 0,
                               rows: [0; 5],
                               cols: [0; 5],
                               won: false,
                               times_to_win: 0,
                               last_num_called: 0};

        let mut row : u8 = 0;
        let mut col : u8 = 0;

        for split in board.split(' ') {
            if split.len() == 0 {
                continue;
            }

            // Add stuff here
            //println!("col {} row {}", col, row);
            //println!("split {}", split);

            let split = split.parse::<usize>().expect("Unable to parse board value!");

            ret.score += split as u16; // Added value to the board
            let cell = &mut ret.cells[split];

            if cell.valid {
                panic!("2 cells!");
            }

            cell.row = row;
            cell.col = col;
            cell.valid = true;

            //println!("Marked cell ({},{}) = {}", row, col, split);

            col += 1;
            if col == 5 {
                row += 1;
                col = 0;
                //print!("\n");
            }
        }

        //println!("Max score ret {}", ret.score);

        ret
    }
}

fn splitter(marks: &String) {
    for split in marks.split(',') {
        println!("Splitted {}", split);
    }
}

fn main() {
    // Since we have only 2 digit numbers, we'll use a simple array
    // to store the board
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let marks = lines.next().expect("no lines in buffer").expect("failed to read line.");

    splitter(&marks);

    let mut temp_bingoboard : BingoBoard;
    let mut count = 0;
    let mut board_index : u32 = 0;
    let mut temp_str : String = String::new();
    let mut best_win_time : u16 = 65535;
    let mut curr_score : u32 = 0;

    for line in lines {
        let line_content = line.unwrap();
        //println!("Line: {}", line_content);

        if line_content.len() == 0 {
            continue;
        }

        count += 1;

        temp_str.push_str(&line_content);
        temp_str.push_str(" ");

        if count == 5 {
            board_index += 1;

            // Calculate all the stuff here
            temp_bingoboard = BingoBoard::init(&temp_str);
            temp_bingoboard.apply_marks(&marks);

            if temp_bingoboard.won {
                println!("Board {} won! Score {}. times {}. lnc {}. res {}.", board_index,
                                                                    temp_bingoboard.score as u32,
                                                                    temp_bingoboard.times_to_win,
                                                                    temp_bingoboard.last_num_called,
                                                                    temp_bingoboard.score as u32 * temp_bingoboard.last_num_called as u32);
                if temp_bingoboard.times_to_win < best_win_time {
                    best_win_time = temp_bingoboard.times_to_win;
                    curr_score = temp_bingoboard.score as u32 * temp_bingoboard.last_num_called as u32;
                }
            }

            count = 0;
            temp_str = String::new();
        }
    }

    println!("Result {}", curr_score);
}
