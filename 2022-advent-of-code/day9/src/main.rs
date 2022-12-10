use std::{fs::File, io::Read, collections::HashMap};

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

fn conv_movement(offset: i32) -> i32 {
    if offset.is_positive() {
        return 1
    } else if offset.is_negative() {
        return -1
    }

    return 0
}

fn get_new_tail_pos(head: &Position, tail: &Position) -> Position {
    let x_offset = head.x - tail.x;
    let y_offset = head.y - tail.y;

    let mut new_tail_pos = Position::new(tail.x, tail.y);

    if x_offset.abs() > 1 {
        new_tail_pos.x += conv_movement(x_offset);

        if y_offset.abs() > 0 {
            new_tail_pos.y += conv_movement(y_offset);
        }
    } else if y_offset.abs() > 1 {
        new_tail_pos.y += conv_movement(y_offset);

        if x_offset.abs() > 0 {
            new_tail_pos.x += conv_movement(x_offset);
        }
    }

    new_tail_pos
}

fn parse_line(line: &str) -> Direction {
    let mut line_split = line.split_whitespace();
    let direction = line_split.next().unwrap();
    let magnitude = line_split.next().unwrap().parse::<i32>().unwrap();

    match direction {
        "R" => Direction::Right(magnitude),
        "L" => Direction::Left(magnitude),
        "U" => Direction::Up(magnitude),
        "D" => Direction::Down(magnitude),
        d => panic!("Invalid direction: {}", d)
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

    let input_vec: Vec<&str> = input_str.lines()
        .collect();

    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);

    // Hashmap to store positions
    let mut tail_positions: HashMap<Position, i32> = HashMap::new();

    // Add the starting position
    tail_positions.insert(tail, 0);

    // Parse inputs
    for line in input_vec {
        let dir = parse_line(line);

        let magnitude = match dir {
            Direction::Up(m) => m,
            Direction::Down(m) => m,
            Direction::Left(m) => m,
            Direction::Right(m) => m,
        };

        for _ in 0..magnitude {
            match dir {
                Direction::Up(m) => head.y -= conv_movement(m),
                Direction::Down(m) => head.y += conv_movement(m),
                Direction::Left(m) => head.x -= conv_movement(m),
                Direction::Right(m) => head.x += conv_movement(m),
            }

            tail = get_new_tail_pos(&head, &tail);
            let curr_val = tail_positions.get(&tail);

            match curr_val {
                Some(val) => tail_positions.insert(tail, val+1),
                None => tail_positions.insert(tail, 1),
            };
        }
    }

    // Report all positions
    let num_pos = tail_positions.len();

    println!("Number of positions (p1): {}", num_pos);
}
