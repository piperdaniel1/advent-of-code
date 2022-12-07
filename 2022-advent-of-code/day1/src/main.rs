use std::fs::File;
use std::io::Read;

struct ElfBackpack {
    pub items: Vec<i32>
}

impl ElfBackpack {
    pub fn get_total_cals(&self) -> i32 {
        self.items.iter().sum()
    }
}

trait Argmax {
    fn argmax(&self) -> usize;
}

trait Max {
    fn max(&self) -> i32;
}

impl Argmax for Vec<ElfBackpack> {
    fn argmax(&self) -> usize {
        let mut max = 0;
        let mut max_idx = 0;
        for (idx, item) in self.iter().enumerate() {
            if item.get_total_cals() > max {
                max = item.get_total_cals();
                max_idx = idx;
            }
        }
        max_idx
    }
}

impl Max for Vec<ElfBackpack> {
    fn max(&self) -> i32 {
        let mut max = 0;
        for item in self.iter() {
            if item.get_total_cals() > max {
                max = item.get_total_cals();
            }
        }
        max
    }
}


fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_lines_str = String::new();
    input_file.read_to_string(&mut input_lines_str).unwrap();

    let input_lines: Vec<&str> = input_lines_str
        .split("\n")
        .collect();

    let mut backpacks: Vec<ElfBackpack> = Vec::new();
    let mut items: Vec<i32> = Vec::new();

    for line in input_lines {
        if line == "" {
            backpacks.push(ElfBackpack { items: items.clone() });
            items = Vec::new();
        } else {
            items.push(line.parse::<i32>().unwrap());
        }
    }

    // Add the last backpack
    backpacks.push(ElfBackpack { items: items.clone() });

    // Part 1 - Find Elf with the most calories
    let highest_cal_idx = backpacks.argmax();
    let highest_cal = backpacks.max();

    println!("Elf with the most calories: #{} with {} calories", highest_cal_idx, highest_cal);

    backpacks.sort_by(|a, b| b.get_total_cals().cmp(&a.get_total_cals()));

    let mut total = 0;
    for i in 0..3 {
        println!("{}", backpacks[i].get_total_cals());
        total += backpacks[i].get_total_cals();
    }

    println!("Total: {}", total);
}
