use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn main() -> Result<()>{
    let test_mode = false;
    let mut input_file: File;

    if !test_mode {
        input_file = File::open("input.txt")?;
    } else {
        input_file = File::open("input_test_case.txt")?;
    }

    let mut input_str = String::new();
    input_file.read_to_string(&mut input_str)?;
    let input_lines = input_str.lines();

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input_lines {
        let line_vec = line.split(" ").collect::<Vec<&str>>();
        let line_mag = line_vec[1].parse::<i32>().unwrap();

        if line_vec[0] == "forward" {
            if test_mode {
                println!("Forward {}", line_mag);
            }
            horizontal_pos += line_mag;
            depth += aim*line_mag;
        } else if line_vec[0] == "up" {
            if test_mode {
                println!("Up {}", line_mag);
            }
            aim -= line_mag;
        } else if line_vec[0] == "down" {
            if test_mode {
                println!("Down {}", line_mag);
            }
            aim += line_mag;
        }
    }

    println!("Horizontal Position: {}", horizontal_pos);
    println!("Depth: {}", depth);

    println!("Depth * Horizontal Pos = {}", depth * horizontal_pos);

    Ok(())
}
