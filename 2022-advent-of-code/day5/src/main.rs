use std::{fs::File, io::Read};

// 1-indexed stack_num returns 0-index column
// 1 2 3 4
// 1 5 9 13
fn get_stack_col(stack_num: i32) -> i32 {
    return ((stack_num-1) * 4) + 1;
}

fn get_num_stacks(stack_line: &str) -> i32 {
    let mut num = 1;
    loop {
        match stack_line.chars().nth(get_stack_col(num) as usize) {
            Some(_) => num += 1,
            None => return num-1,
            _ => num += 1,
        }
    }
} fn main() {
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

    let input_vec: Vec<&str> = input_str
        .lines()
        .collect();

    let num_stacks = get_num_stacks(&input_vec[0]);
    println!("num_stacks: {}", num_stacks);

    // Initialize stacks to empty
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut p2_stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
        p2_stacks.push(Vec::new());
    }


    let mut init_mode = true;
    for line in input_vec {
        if init_mode {
            for i in 0..num_stacks {
                let col = get_stack_col(i+1);
                let stack_char = line.chars().nth(col as usize).unwrap();
                if stack_char >= '0' && stack_char <= '9' {
                    init_mode = false;
                    break;
                } else if stack_char != ' ' {
                    stacks[i as usize].insert(0, stack_char);
                    p2_stacks[i as usize].insert(0, stack_char);
                }
            }
        } else {
            let space_split = line.split(" ")
                .collect::<Vec<&str>>();
            
            if space_split.len() != 6 {
                continue;
            }

            let num_crates = space_split[1].parse::<i32>().unwrap();
            let from_stack = space_split[3].parse::<i32>().unwrap();
            let to_stack = space_split[5].parse::<i32>().unwrap();

            for i in 0..num_crates {
                let crate_char = stacks[(from_stack-1) as usize].pop().unwrap();
                stacks[(to_stack-1) as usize].push(crate_char);

                let ind_p2 = p2_stacks[(from_stack-1) as usize].len() - (num_crates as usize - i as usize);
                let p2_crate_char = p2_stacks[(from_stack-1) as usize].remove(ind_p2);
                p2_stacks[(to_stack-1) as usize].push(p2_crate_char);
            }
        }
    }

    let mut output = String::new();
    for i in 0..num_stacks {
        output.push(stacks[i as usize][stacks[i as usize].len()-1]);
    }

    println!("Top Crates: {}", output);
    
    let mut p2_output = String::new();
    for i in 0..num_stacks {
        p2_output.push(p2_stacks[i as usize][p2_stacks[i as usize].len()-1]);
    }

    println!("Top Crates (Part 2): {}", p2_output);
}
