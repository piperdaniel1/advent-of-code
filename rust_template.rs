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
}
