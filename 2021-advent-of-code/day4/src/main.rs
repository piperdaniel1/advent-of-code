use std::collections::VecDeque;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::vec::Vec;
use std::time::{SystemTime};

#[derive(Debug)]
struct BingoNums {
    nums: VecDeque<i32>,
}
impl Default for BingoNums {
    fn default() -> Self {
        BingoNums {
            nums: VecDeque::new(),
        }
    }
}
impl BingoNums {
    fn get_next_num(&mut self) -> i32 {
        let next_num = self.nums.pop_front();

        if next_num.is_none() {
            panic!("No more numbers");
        } else {
            return next_num.unwrap();
        }
    }

    fn get_recent_num(&self) -> i32 {
        return self.nums[self.nums.len()-1];
    }

    fn contains(&self, num: i32) -> bool {
        return self.nums.contains(&num);
    }

    fn add_num(&mut self, num: i32) {
        self.nums.push_back(num);
    }
}

#[derive(Debug)]
struct BingoBoard {
    arr_2d: [[i32; 5]; 5],
    is_default: bool,
}
impl Default for BingoBoard {
    fn default() -> BingoBoard {
        BingoBoard {
            arr_2d: [[0; 5]; 5],
            is_default: true,
        }
    }
}
impl BingoBoard {
    fn mutate_square(&mut self, row: usize, col: usize, val: i32) {
        self.arr_2d[row][col] = val;
        self.is_default = false;
    }

    fn print(&self) {
        for i in 0..5 {
            for j in 0..5 {
                let length = self.arr_2d[i][j].to_string().len();
                if length == 2 {
                    print!("{} ", self.arr_2d[i][j]);
                } else {
                    print!("{}  ", self.arr_2d[i][j]);
                }
            }
            println!();
        }
    }

    fn calc_score(&self, nums: &BingoNums) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !nums.contains(self.arr_2d[i][j]) {
                    sum += self.arr_2d[i][j];
                }
            }
        }

        return sum * nums.get_recent_num();
    }

    fn has_bingo(&self, nums: &BingoNums, count_diagonals: bool) -> bool {
        // check for vertical bingos
        let mut found = true;
        for j in 0..5 {
            for i in 0..5 {
                if !nums.contains(self.arr_2d[i][j]) {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
            found = true;
        }
        
        // check for horizontal bingos
        for i in 0..5 {
            for j in 0..5 {
                if !nums.contains(self.arr_2d[i][j]) {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
            found = true;
        }
        
        if count_diagonals {
            // check for diagonal bingos
            for i in 0..5 {
                if !nums.contains(self.arr_2d[i][i]) {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
            found = true;

            for i in 0..5 {
                if !nums.contains(self.arr_2d[4-i][i]) {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
        }

        // return false if we did find any bingos
        return false;
    }
}

fn main() -> Result<()> {
    let start_time = SystemTime::now();
    let mut input_file = File::open("input_test_case.txt")?;
    let mut input_lines_str = String::new();
    input_file.read_to_string(&mut input_lines_str)?;

    let input_lines: Vec<&str> = input_lines_str.split('\n').collect();
    let mut nums_store = BingoNums {..Default::default()};

    let bingo_nums = input_lines[0].split(',');

    for num_str in bingo_nums {
        // The unwrap call here gets rid of the parse error.
        // I assume it gets rid of it by returning a bad result
        // from main instantly but idk exactly
        let num = num_str.parse::<i32>().unwrap();
        nums_store.add_num(num);
    }

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut board = BingoBoard {..Default::default()};
    for line in input_lines.iter().skip(1) {
        if line.len() == 0 {
            row = 0;
            col = 0;

            if !board.is_default {
                boards.push(board);
            }
            board = BingoBoard {..Default::default()};
            continue;
        }

        let row_nums = line.split(' ');

        for num_str in row_nums {
            // sometimes there are double spaces for formatting,
            // these should not be added
            if num_str.len() == 0 {
                continue;
            }

            // Other way of handling errors
            let num = num_str.parse::<i32>();
            let num = match num {
                Ok(number) => number,
                Err(_) => panic!("Integer parse error: '{}'", num_str),
            };
            board.mutate_square(row, col, num);
            col += 1;
        }

        row += 1;
        col = 0;
    }

    let mut new_bingo_nums = BingoNums {..Default::default()};

    // Loop until one of the boards has a bingo.
    while boards.len() > 1 {
        new_bingo_nums.add_num(nums_store.get_next_num());

        boards.retain(|board| {
            if board.has_bingo(&new_bingo_nums, false) {
                return false;
            }
            return true;
        });
    }

    while !boards[0].has_bingo(&new_bingo_nums, false) {
        new_bingo_nums.add_num(nums_store.get_next_num());
    }

    println!("Score of last board to win: {}", boards[0].calc_score(&new_bingo_nums));

    let total_time = SystemTime::now().duration_since(start_time)
        .expect("Time is a construct.");
    
    println!("Time taken: {:#?}", total_time);

    Ok(())
}
