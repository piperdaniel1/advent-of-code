use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

pub trait Sum {
    fn get_sum(&self) -> i32;
}

impl Sum for VecDeque<i32> {
    fn get_sum(&self) -> i32 {
        let mut sum = 0;
        for num in self {
            sum += num;
        }
        return sum;
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines();

    let mut last_line: i32 = -1; 
    let mut sliding_window: VecDeque<i32> = VecDeque::new();
    let mut num_increases: i32 = 0;
    let mut last_window_sum: i32 = -1;
    let mut window_increases: i32 = 0;

    for line in lines {
        let line = line.to_string();
        let line_num = line.parse::<i32>().unwrap();

        sliding_window.push_back(line_num);
        if sliding_window.len() > 3 {
            sliding_window.pop_front();
        }
        
        if last_line != -1 {
            if line_num > last_line {
                num_increases += 1;
            }
        }
        last_line = line_num;

        let mut current_window_sum = -1;
        if sliding_window.len() == 3 {
            current_window_sum = sliding_window.get_sum();
        }

        if last_window_sum != -1 {
            if current_window_sum > last_window_sum {
                window_increases += 1;
            }
        }

        last_window_sum = current_window_sum;
    }

    println!("The depth numbers increased {} times", num_increases);
    println!("The window increased {} times", window_increases);
    Ok(())
}
