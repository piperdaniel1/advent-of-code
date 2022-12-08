use std::{fs::File, io::Read};

fn get_common_letter(box1: &String, box2: &String) -> char {
    for c1 in box1.chars() {
        for c2 in box2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }
    panic!("No common letter found");
}

fn get_group_badge(box1: &str, box2: &str, box3: &str) -> char {
    for c1 in box1.chars() {
        let mut c2_found = false;
        for c2 in box2.chars() {
            if c1 == c2 {
                c2_found = true;
                break;
            }
        }

        if !c2_found {
            continue;
        }

        
        for c3 in box3.chars() {
            if c1 == c3 {
                return c1;
            }
        }
    }
     
    panic!("No badge found");
}

// a-z = 1-26, A-Z = 27-52
fn get_score(letter: char) -> u32 {
    let mut score = letter as u32;
    if score > 96 {
        score -= 96;
    } else {
        score -= 38;
    }
    score
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
    let input_vec: Vec<(String, String)> = input_str.lines()
        .map(|line| (line[0..line.len()/2].to_string(), line[line.len()/2..].to_string()))
        .collect();

    /* Part One */
    let mut priority = 0;
    for (box1, box2) in input_vec.iter() {
        if debug {
            println!("---------------------");
            println!("{} {}", box1, box2);
        }

        let common = get_common_letter(box1, box2);
        if debug {
            println!("Common letter: {} (score={})", common, get_score(common));
        }

        priority += get_score(common);
    }
    println!("Total Priority: {}", priority);

    /* Part Two */
    let group_vec: Vec<&str> = input_str.lines()
        .collect();

    assert!(group_vec.len() % 3 == 0);

    let mut score = 0;
    for i in 0..group_vec.len()/3 {
        if debug {
            println!("-------------------");
            println!("Group member 1: {}", group_vec[i*3]);
            println!("Group member 2: {}", group_vec[i*3 + 1]);
            println!("Group member 3: {}", group_vec[i*3 + 2]);
        }

        let badge = get_group_badge(group_vec[i*3], group_vec[i*3 + 1], group_vec[i*3 + 2]);
        if debug {
            println!("Badge: {} (score={})", badge, get_score(badge));
        }
        score += get_score(badge);
    }

    println!("Total Badge Priority: {}", score);
}
