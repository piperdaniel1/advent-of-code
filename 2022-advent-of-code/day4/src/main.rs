use std::{fs::File, io::Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = if args.len() > 1 {
        args[1].clone()
    } else {
        "input.txt".to_string()
    };

    let debug = if args.len() > 2 {
        args[2].clone() == "debug"
    } else {
        false
    };

    let mut input = File::open(filename).unwrap();
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();
    let input_vec: Vec<((i32, i32), (i32, i32))> = input_str
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            let left = left
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let right = right
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            ((left[0], left[1]), (right[0], right[1]))
        })
        .collect();

    let mut overlaps = 0;
    for line in &input_vec {
        // First elf is in the range of the second elf
        if line.0.0 >= line.1.0 && line.0.1 <= line.1.1 {
            overlaps += 1;
        }
        // Second elf is in the range of the first elf
        else if line.1.0 >= line.0.0 && line.1.1 <= line.0.1 {
            overlaps += 1;
        }
    }

    println!("Overlaps: {}", overlaps);

    let mut partial_overlaps = 0;
    for line in &input_vec {
        // First elf starts during the second elf
        if line.0.0 >= line.1.0 && line.0.0 <= line.1.1 {
            partial_overlaps += 1;
        // Second elf starts during the first elf
        } else if line.1.0 >= line.0.0 && line.1.0 <= line.0.1 {
            partial_overlaps += 1;
        }
    }

    println!("Partial overlaps: {}", partial_overlaps);
}
