use std::{fs::File, io::Read};

#[derive(Debug)]
struct FileEntry {
    pub name: String,
    pub size: u64,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<FileEntry>,
    directories: Vec<Directory>,
}

#[derive(Debug)]
enum CommandBase {
    CD,
    LS,
}

#[derive(Debug)]
struct Command {
    base: CommandBase,
    args: Vec<String>,
}

impl Command {
    fn new(base: CommandBase, args: Vec<String>) -> Command {
        Command {
            base,
            args,
        }
    }
}

impl Directory {
    fn new(name: impl ToString) -> Directory {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn add_file(&mut self, name: String, size: u64) {
        self.files.push(FileEntry { name, size });
    }

    fn add_directory(&mut self, name: String) {
        self.directories.push(Directory::new(name));
    }

    fn get_size(&self) -> u64 {
        let mut size = 0;

        for f in &self.files {
            size += f.size;
        }

        for d in &self.directories {
            size += d.get_size();
        }

        return size;
    }

    fn get_directory(&mut self, name: &str) -> Option<&mut Directory> {
        for d in &mut self.directories {
            if d.name == name {
                return Some(d);
            }
        }

        return None;
    }
}

#[derive(Debug)]
enum ParseResult {
    Command(Command),
    FileEntry(FileEntry),
    Directory(Directory),
    ParseError,
}

fn parse_line(line: &str) -> ParseResult {
    if line.len() < 1 {
        return ParseResult::ParseError;
    }

    let cmd_line: Vec<char> = line.chars().into_iter().collect();

    // Command
    if cmd_line[0] == '$' {
        let cmd_line: String = cmd_line[2..].into_iter().collect();

        let space_split = cmd_line.split(' ').collect::<Vec<&str>>();

        let base = match space_split[0] {
            "cd" => CommandBase::CD,
            "ls" => CommandBase::LS,
            _ => return ParseResult::ParseError,
        };

        let args: Vec<String> = space_split[1..].into_iter().map(|s| s.to_string()).collect();

        return ParseResult::Command(Command::new(base, args));

    // Directory
    } else if cmd_line[0..3] == ['d', 'i', 'r'] {
        let dir_name: String = cmd_line[4..].into_iter().collect();
        return ParseResult::Directory(Directory::new(dir_name));
    }

    // File
    else {
        let cmd_string: String = cmd_line.into_iter().collect();

        let space_split = cmd_string.split(' ').collect::<Vec<&str>>();

        let size = space_split[0].parse::<u64>().unwrap();
        let name = space_split[1].to_string();

        return ParseResult::FileEntry(FileEntry { name, size });
    }
}

// Includes the starting directory
fn rec_search(dir: &mut Directory, threshold: u64) -> u64 {
    let mut size = 0;
    let c_size = dir.get_size();

    if c_size <= threshold {
        size += c_size;
    }

    for d in &mut dir.directories {
        size += rec_search(d, threshold);
    }

    return size;
}

fn find_smallest_above_threshold(dir: &mut Directory, threshold: u64, mut curr_best: u64) -> u64 {
    if dir.get_size() > threshold && dir.get_size() < curr_best {
        curr_best = dir.get_size();
    }

    for d in &mut dir.directories {
        let new_best = find_smallest_above_threshold(d, threshold, curr_best);

        if new_best < curr_best {
            curr_best = new_best;
        }
    }

    return curr_best;
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
    let input_lines: Vec<&str> = input_str.lines()
        .collect();

    let mut root = Directory::new("root");
    let mut curr_path: Vec<String> = Vec::new();
    let mut curr_node = &mut root;

    for line in input_lines {
        let parsed_line = parse_line(line);

        match parsed_line {
            ParseResult::Command(cmd) => {
                match cmd.base {
                    CommandBase::CD => {
                        let to_dir = cmd.args[0].clone();
                        if to_dir == ".." {
                            curr_path.pop();
                            curr_node = &mut root;
                            for dir in &curr_path {
                                curr_node = curr_node.get_directory(dir).unwrap();
                            }
                        } else if to_dir == "/" {
                            curr_path = Vec::new();
                            curr_node = &mut root;
                        } else {
                            curr_path.push(to_dir.clone());
                            curr_node = curr_node.get_directory(&to_dir).unwrap();
                        }
                    },
                    CommandBase::LS => {
                        continue;
                    },
                }
            },
            ParseResult::FileEntry(file) => {
                curr_node.add_file(file.name, file.size);
            },
            ParseResult::Directory(dir) => {
                curr_node.add_directory(dir.name.clone());
            },
            ParseResult::ParseError => {
                panic!("Parse error on line: {}", line);
            },
        }
    }



    let p1_size = rec_search(&mut root, 100000);
    println!("Part 1: {}", p1_size);

    let fs_size = root.get_size();
    let total_space: u64 = 70_000_000;
    let update_size = 30_000_000;
    let amt_available = total_space - fs_size;
    let threshold = update_size - amt_available;

    let p2_size = find_smallest_above_threshold(&mut root, threshold, u64::max_value());
    println!("Part 2: {}", p2_size);
}
