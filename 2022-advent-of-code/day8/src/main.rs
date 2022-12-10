use std::{fs::File, io::Read, fmt::Display};

#[derive(Debug)]
struct TreeMap {
    map: Vec<Vec<u8>>,
}

struct Square {
    x: usize,
    y: usize,
}

struct Direction {
    x: i32,
    y: i32,
}

impl TreeMap {
    fn new(map: Vec<Vec<u8>>) -> TreeMap {
        TreeMap {
            map,
        }
    }

    fn get(&self, square: &Square) -> u8 {
        self.map[square.y][square.x]
    }

    fn is_visible(&self, square: &Square) -> bool {
        // Bottom row
        if square.y >= self.map.len()-1 {
            return true;
        // Top row
        } else if square.y <= 0 {
            return true
        // Right side
        } else if square.x >= self.map[square.y].len()-1 {
            return true;
        // Left side
        } else if square.x <= 0 {
            return true;
        }

        let tree_value = self.get(square);

        // Scan from the left side to our tree
        let mut left_taller = false;
        for i in 0..square.x {
            let left_square = Square {
                x: i,
                y: square.y,
            };
            let left_value = self.get(&left_square);
            if left_value >= tree_value {
                left_taller = true;
                break;
            }
        }

        // Scan from the right side to our tree
        let mut right_taller = false;
        for i in square.x+1..self.map[square.y].len() {
            let right_square = Square {
                x: i,
                y: square.y,
            };
            let right_value = self.get(&right_square);
            if right_value >= tree_value {
                right_taller = true;
                break;
            }
        }

        // Scan from the top to our tree
        let mut top_taller = false;
        for i in 0..square.y {
            let top_square = Square {
                x: square.x,
                y: i,
            };
            let top_value = self.get(&top_square);
            if top_value >= tree_value {
                top_taller = true;
                break;
            }
        }

        // Scan from the bottom to our tree
        let mut bottom_taller = false;
        for i in square.y+1..self.map.len() {
            let bottom_square = Square {
                x: square.x,
                y: i,
            };
            let bottom_value = self.get(&bottom_square);
            if bottom_value >= tree_value {
                bottom_taller = true;
                break;
            }
        }

        if left_taller && right_taller && top_taller && bottom_taller {
            return false;
        }

        true
    }

    fn count_visible(&self, square: &Square, direction: Direction) -> i32 {
        let mut count = 0;
        let height = self.get(square);

        for i in 1.. {
            let new_x = square.x as i32 + (direction.x * i);
            let new_y = square.y as i32 + (direction.y * i);

            if new_x < 0 || new_y < 0 {
                break;
            }

            if new_y as usize >= self.map.len() {
                break;
            }
            if new_x as usize >= self.map[new_y as usize].len() {
                break;
            }

            let curr_square = Square {
                x: new_x as usize,
                y: new_y as usize,
            };

            if curr_square.x >= self.map[curr_square.y].len() {
                break;
            }

            if curr_square.y >= self.map.len() {
                break;
            }

            let curr_height = self.get(&curr_square);
            if curr_height < height {
                count += 1;
            } else {
                count += 1;
                break;
            }
        }

        count
    }

    fn calc_scenic_score(&self, square: &Square, debug: bool) -> i32 {
        let mut score = 0;
        // Left
        let res = self.count_visible(square, Direction { x: -1, y: 0 });
        score += res;
        if debug {
            println!("Left: {}", res);
        }

        // Right
        let res = self.count_visible(square, Direction { x: 1, y: 0 });
        score *= res;
        if debug {
            println!("Right: {}", res);
        }

        // Up
        let res = self.count_visible(square, Direction { x: 0, y: -1 });
        score *= res;
        if debug {
            println!("Up: {}", res);
        }

        // Down
        let res = self.count_visible(square, Direction { x: 0, y: 1 });
        score *= res;
        if debug {
            println!("Down: {}", res);
        }

        score
    }
}

impl Display for TreeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.map.len() == 0 {
            writeln!(f, "TreeMap: {} rows", self.map.len())?;
        } else {
            writeln!(f, "TreeMap: {} rows, {} columns", self.map.len(), self.map[0].len())?;
        }
        for row in &self.map {
            for col in row {
                write!(f, "{} ", *col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Square {
    fn new(x: i32, y: i32) -> Square {
        Square {
            x: x as usize,
            y: y as usize,
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

    let input_vec: Vec<&str> = input_str.lines()
        .collect();

    // Parse out the tree map
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in input_vec {
        map.push(Vec::new());
        let line_chars: Vec<u8> = line.chars().map(|x| x as u8).collect();
        for c in line_chars {
            map.last_mut().unwrap().push(c - 48);
        }
    }

    let tree_map = TreeMap::new(map);
    println!("{}", tree_map);

    // this is the least efficient code i've ever written

    let mut total_visible = 0;

    for y in 0..tree_map.map.len() {
        for x in 0..tree_map.map[y].len() {
            let square = Square::new(x as i32, y as i32);
            let is_visible = tree_map.is_visible(&square);
            if is_visible {
                print!("V ");
                total_visible += 1;
            } else {
                print!("X ");
            }
        }
        println!();
    }

    println!("Total visible: {}", total_visible);

    for y in 0..tree_map.map.len() {
        for x in 0..tree_map.map[y].len() {
            let square = Square::new(x as i32, y as i32);
            let score = tree_map.calc_scenic_score(&square, false);
            print!("{} ", score);
        }
        println!();
    }

    let test_square = Square::new(2, 3);
    let score = tree_map.calc_scenic_score(&test_square, true);
    println!("Score: {}", score);

    

}
