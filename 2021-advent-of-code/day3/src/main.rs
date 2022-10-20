use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

/*
 * Post Puzzle Thoughts:
 * It was really cool how the Vec.retain() thing works.
 * It really reminded me of being able to do a simple list comprehension in Python.
 *
 * For example in Python I could have done:
 * new_list = [x for x in old_list if x[bit_index] != 0]
 * This way is definitely less Pythonic and a bit less versatile maybe but I do like that
 * it exists at all. Definitely would not exist in C for example. You would be stuck using a for
 * loop to iterate over the char** removing items like a caveman.
 *
 * The AsDecimal trait was a bit weird. I do not really like how it will Panic! if there is
 * not a 0 or 1. But it got the job done I guess.
 *
 * Getting a string of the file and then splitting it into a Split was interesting. Not sure
 * what the Split does exactly. However, it is nice because I can easily .collect() it into
 * a vector. Now that I think about it the Split is probably an Iterator of some sort.
 *
 * The second part should definitely be refactored. There is no reason to find both the scrubber
 * and the oxygen at the same time it should have been seperate loops for each.
 *
 * Finally, it was weird to have to use as_bytes(). However, I guess the reason for this is that
 * Rust supports UTF-8 natively. I cannot really fault this I guess. However, it definitely makes
 * everything a bit more tricky when I am just working with ascii strings.
 *
 * I believe that as_bytes() is a zero cost abstraction so it does not really matter from a
 * performance standpoint. One tricky thing is that as_bytes() makes the str into a bunch of u8s
 * not chars so that means that you have to cast each char you want to compare as u8 like this: 
 * 'c' as u8. Again, probably zero cost at runtime.
 */

pub trait AsDecimal {
    fn binary_to_dec(&self) -> i32;
}

impl AsDecimal for String {
    /*
     * This is a very sketchy function
     */
    fn binary_to_dec(&self) -> i32 {
        let mut pos: i32 = self.len() as i32 - 1;
        let mut answer = 0;

        for c in self.chars() {
            if c == '0' {
                pos -= 1;
            } else if c == '1' {
                answer += (2 as i32).pow(pos as u32);
                pos -= 1;
            } else {
                panic!("String contains something other than a 1 or a 0");
            }
        }
        return answer;
    }
}

fn main() -> std::io::Result<()> {
    // Open input.txt file
    let mut input_file = File::open("input.txt")?;
    // Create string and read file into it
    let mut input_str = String::new();
    input_file.read_to_string(&mut input_str)?;

    // Split string into Split<&str> by line
    let input_lines = input_str.split("\n");

    // Collect Split into a Vec<&str>
    let input_line_vec: Vec<&str> = input_lines.collect();

    /*
     * 
     * Part One Calculations
     *
     */
    let line_length = input_line_vec[0].as_bytes().len();
    
    let mut bit_counts = vec![0; line_length];

    for line_ind in 0..input_line_vec.len() {
        if input_line_vec[line_ind].len() == 0 {
            continue;
        }
        let line_bytes = input_line_vec[line_ind].as_bytes();

        for i in 0..line_length {
            if line_bytes[i] == '1' as u8 {
                bit_counts[i] += 1;
            } else {
                bit_counts[i] -= 1;
            }
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for elem in bit_counts {
        if elem > 0 {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }

    let gamma_dec = gamma_rate.binary_to_dec();
    let epsilon_dec = epsilon_rate.binary_to_dec();
    println!("Gamma Rate: {} = {}", gamma_rate, gamma_rate.binary_to_dec());
    println!("Epsilon Rate: {} = {}", epsilon_rate, epsilon_rate.binary_to_dec());
    println!("Gamma * Epsilon: {}", gamma_dec * epsilon_dec);

    /*
     *
     * Part Two Calculations
     *
     */
    let mut oxygen_shortlist = input_line_vec.clone();
    let mut scrubber_shortlist = input_line_vec.clone();

    // Remove empty entry on the end
    oxygen_shortlist.pop();
    scrubber_shortlist.pop();

    for bit_index in 0..line_length {
        //println!("Bit index: {}", bit_index);
        let mut zero_count_oxy = 0;
        let mut one_count_oxy = 0;
        for line_ind in 0..oxygen_shortlist.len() {
            if oxygen_shortlist[line_ind].len() == 0 {
                continue;
            }

            if oxygen_shortlist[line_ind].as_bytes()[bit_index] == '0' as u8 {
                zero_count_oxy += 1;
            } else if oxygen_shortlist[line_ind].as_bytes()[bit_index] == '1' as u8 {
                one_count_oxy += 1;
            }
        }

        let mut zero_count_scrub = 0;
        let mut one_count_scrub = 0;
        for line_ind in 0..scrubber_shortlist.len() {
            if scrubber_shortlist[line_ind].len() == 0 {
                continue;
            }

            if scrubber_shortlist[line_ind].as_bytes()[bit_index] == '0' as u8 {
                zero_count_scrub += 1;
            } else if scrubber_shortlist[line_ind].as_bytes()[bit_index] == '1' as u8 {
                one_count_scrub += 1;
            }
        }

        //println!("Zero count oxygen: {}", zero_count_oxy);
        //println!("One count oxygen: {}", one_count_oxy);
        //println!();
        //println!("Zero count scrubber: {}", zero_count_scrub);
        //println!("One count scrubber: {}", one_count_scrub);

        if zero_count_oxy > one_count_oxy {
            if oxygen_shortlist.len() > 1 {
                oxygen_shortlist.retain(|&x| x.as_bytes()[bit_index] == '0' as u8)
            }
        } else {
            if oxygen_shortlist.len() > 1 {
                oxygen_shortlist.retain(|&x| x.as_bytes()[bit_index] == '1' as u8)
            }
        }
        if zero_count_scrub > one_count_scrub {
            if scrubber_shortlist.len() > 1 {
                scrubber_shortlist.retain(|&x| x.as_bytes()[bit_index] == '1' as u8)
            }
        } else {
            if scrubber_shortlist.len() > 1 {
                scrubber_shortlist.retain(|&x| x.as_bytes()[bit_index] == '0' as u8)
            }
        }

        //println!("Narrowed oxygen: {:?}", oxygen_shortlist);
        //println!("Narrowed scrubber: {:?}", scrubber_shortlist);

        if oxygen_shortlist.len() <= 1 && scrubber_shortlist.len() <= 1 {
            break;
        }
    }


    let final_oxygen = oxygen_shortlist[0];
    let final_scrubber= scrubber_shortlist[0];

    let oxygen_dec = final_oxygen.to_string().binary_to_dec();
    let scrubber_dec = final_scrubber.to_string().binary_to_dec();

    println!("Oxygen: {}", oxygen_dec);
    println!("Scrubber: {}", scrubber_dec);
    println!("Oxygen * Scrubber: {}", oxygen_dec * scrubber_dec);
    
    Ok(())
}
