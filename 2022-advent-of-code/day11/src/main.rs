use core::num;
use std::{fs::{File, read_to_string}, io::Read, str::FromStr};

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    ParseErr
}

#[derive(Debug)]
enum Argument {
    Number(i32),
    Old,
}

#[derive(Debug)]
struct Monkey {
    num: i32,
    items: Vec<i32>,
    operator: Operator,
    argument: Argument,
    test_divisor: i32,
    true_path: i32,
    false_path: i32,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            num: 0,
            items: Vec::new(),
            operator: Operator::Add,
            argument: Argument::Number(0),
            test_divisor: 0,
            true_path: 0,
            false_path: 0,
        }
    }
}

impl FromStr for Operator {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Multiply),
            "+" => Ok(Operator::Multiply),
            _ => Err(Operator::ParseErr),
            
        }
    }
}

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

    let line_vec = input_str.lines().collect::<Vec<&str>>();

    let mut curr_monkey = Monkey::default();
    let mut monkey_vec: Vec<Monkey> = Vec::new();
    
    for line in line_vec {
        if line.starts_with("Monkey ") {
            let num_char: i32 = (line.as_bytes()[7] - '0' as u8) as i32;
            curr_monkey.num = num_char;
        } else if line.starts_with("  Starting items: ") {
            let nums_slice = &line[18..line.len()];
            let nums = nums_slice.split(", ")
                .collect::<Vec<&str>>().into_iter()
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            curr_monkey.items = nums;
        } else if line.starts_with("  Operation: new = old ") {
            let operator_char = line.as_bytes()[23];
            let operator_num = line.split_whitespace().nth(6).unwrap();

            let operator = 

        } else if line.starts_with("  Test: divisible by ") {
            
        } else if line.starts_with("    If true: throw to monkey ") {

        } else if line.starts_with("    If false: throw to monkey ") {

        } else {
            monkey_vec.push(curr_monkey);
            curr_monkey = Monkey::default();
        }
    }
}
