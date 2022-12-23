use std::{fs::File, io::Read, fmt::Error};

#[derive(Debug)]
enum Instruction {
    AddToX(i32),
    Wait,
}

fn parse(elem_str: &str) -> Result<Instruction, Error> {
    if elem_str.starts_with("addx") {
        let value: i32 = match elem_str[5..elem_str.len()].parse() {
            Ok(val) => val,
            Err(_) => return Err(Error)
        };

        return Ok(Instruction::AddToX(value))
    } else if elem_str.starts_with("noop") {
        return Ok(Instruction::Wait)
    }

    Err(Error)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = if args.len() > 1 {
        args[1].clone()
    } else {
        "input.txt".to_string()
    };

    let _debug = if args.len() > 2 {
        args[2].clone() == "debug"
    } else {
        false
    };

    let mut input = File::open(filename).unwrap();
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let mut x_reg = 1;
    let mut cycle_num = 0;
    let mut target_cycle = 20;

    let mut pix_row = 0;
    let mut pix_col = 0;

    let input_lines: Vec<&str> = input_str.lines().collect();

    let instructions: Vec<Instruction> = input_lines
        .into_iter()
        .map(|elem| parse(elem)
        .unwrap())
        .collect();

    let mut strength_vec: Vec<i32> = Vec::new();
    
    for instruction in instructions {
        match instruction {
            Instruction::AddToX(val) => {
                cycle_num += 2;

                if cycle_num == target_cycle || cycle_num - 1 == target_cycle {
                    // println!("Cycle {}: Strength is {}*{}={}", target_cycle, target_cycle, x_reg, target_cycle*x_reg);
                    strength_vec.push(target_cycle*x_reg);
                    target_cycle += 40;
                }

                for _ in 0..2 {
                    if pix_col >= x_reg - 1 && pix_col <= x_reg + 1 {
                        print!("#");
                    } else {
                        print!(".")
                    }
                    pix_col += 1;
                    if pix_col == 40 {
                        pix_col = 0;
                        pix_row += 1;
                        println!("");
                    }
                }

                x_reg += val;
            },
            Instruction::Wait => {
                cycle_num += 1;

                if cycle_num == target_cycle {
                    // println!("Cycle {}: Strength is {}", cycle_num, cycle_num*x_reg);
                    strength_vec.push(cycle_num*x_reg);
                    target_cycle += 40;
                }

                if pix_col >= x_reg - 1 && pix_col <= x_reg + 1 {
                    print!("#");
                } else {
                    print!(".")
                }
                pix_col += 1;
                if pix_col == 40 {
                    pix_col = 0;
                    pix_row += 1;
                    println!("");
                }
            },
        }
    }

    println!("Part One: {}", strength_vec.into_iter().sum::<i32>())

}