use std::{fs::File, io::Read};

struct Round {
    pub p1_play: String,
    pub p2_play: String,
}

impl Round {
    pub fn new(p1_play: String, p2_play: String) -> Round {
        Round {
            p1_play,
            p2_play,
        }
    }

    pub fn new_from_string(s: &str) -> Round {
        let mut split = s.split_whitespace();
        Round::new(split.next().unwrap().to_string(), split.next().unwrap().to_string())
    }

    fn get_play_score(&self, play: &String) -> i32 {
        match play.as_str() {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            e => panic!("Invalid play: {}", e),
        }
    }

    pub fn get_p2_score(&self) -> i32 {
        let mut score = 0;

        score += self.get_play_score(&self.p2_play);

        // Draw
        if self.p2_play.draws(&self.p1_play) {
            score += 3;
        }

        // Wins
        else if self.p2_play.beats(&self.p1_play) {
            score += 6;
        }

        // Loses
        else if self.p2_play.loses(&self.p1_play) {
            score += 0;
        }

        return score;
    }
}

trait Beats {
    fn beats(&self, other: &String) -> bool;
}

trait Draws {
    fn draws(&self, other: &String) -> bool;
}

trait Loses {
    fn loses(&self, other: &String) -> bool;
}

impl Beats for String {
    fn beats(&self, other: &String) -> bool {
        match self.as_str() {
            "A" => other == "C",
            "X" => other == "Z",
            "B" => other == "A",
            "Y" => other == "X",
            "C" => other == "B",
            "Z" => other == "Y",
            _ => panic!("Invalid play"),
        }
    }
}

impl Draws for String {
    fn draws(&self, other: &String) -> bool {
        self == other
    }
}

impl Loses for String {
    fn loses(&self, other: &String) -> bool {
        !self.beats(other) && !self.draws(other)
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

    let round_vec = input_str
        .lines()
        .map(|line| Round::new_from_string(line))
        .collect::<Vec<Round>>();

    let mut total_p2_score = 0;
    for round in round_vec {
        if debug {
            println!("P1: {}, P2: {}", round.p1_play, round.p2_play);
            println!("P2 score: {}", round.get_p2_score());
            println!("----------------");
        }
        total_p2_score += round.get_p2_score();
    }

    println!("Total Score: {}", total_p2_score);
}
