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

    let mut window: Vec<char> = Vec::new();
    let mut window_p2: Vec<char> = Vec::new();
    let mut p1_ind = 0;
    let mut p2_ind = 0;
    for (ind, c) in input_str.chars().enumerate() {
        window.push(c);
        window_p2.push(c);

        if window.len() > 4 {
            window.remove(0);
        }

        if window_p2.len() > 14 {
            window_p2.remove(0);
        }

        if window.len() == 4 {
            let mut all_unique = true;
            for (ind1, i) in window.iter().enumerate() {
                for (ind2, j) in window.iter().enumerate() {
                    if i == j && ind1 != ind2 {
                        all_unique = false;
                        break;
                    }
                }
            }
            if all_unique && p1_ind == 0 {
                p1_ind = ind+1;
            }
        }

        if window_p2.len() == 14 {
            let mut all_unique = true;
            for (ind1, i) in window_p2.iter().enumerate() {
                for (ind2, j) in window_p2.iter().enumerate() {
                    if i == j && ind1 != ind2 {
                        all_unique = false;
                        break;
                    }
                }
            }
            if all_unique && p2_ind == 0 {
                p2_ind = ind+1;
            }
        }
    }

    println!("Part 1: {}", p1_ind);
    println!("Part 2: {}", p2_ind);
}
