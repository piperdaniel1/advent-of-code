use std::{fs::File, io::Read};
use std::fmt::Display;

enum Strategy {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Strategy::Rock => write!(f, "rock"),
            Strategy::Paper => write!(f, "paper"),
            Strategy::Scissors => write!(f, "scissors"),
        }
    }
}

fn string_to_strategy(s: &str) -> Strategy {
    match s {
        "A" => Strategy::Rock,
        "B" => Strategy::Paper,
        "C" => Strategy::Scissors,
        "X" => Strategy::Rock,
        "Y" => Strategy::Paper,
        "Z" => Strategy::Scissors,
        _ => panic!("Invalid strategy"),
    }
}

trait ParseStrategy {
    fn parse_strategy(&self) -> Option<Strategy>;
}
impl ParseStrategy for String {
    fn parse_strategy(&self) -> Option<Strategy> {
        Some(string_to_strategy(self))
    }
}
impl ParseStrategy for &str {
    fn parse_strategy(&self) -> Option<Strategy> {
        Some(string_to_strategy(self))
    }
}
impl ParseStrategy for Option<String> {
    fn parse_strategy(&self) -> Option<Strategy> {
        match self {
            Some(s) => Some(string_to_strategy(s)),
            None => None,
        }
    }
}

trait ParseResult {
    fn parse_result(&self) -> GameResult;
}
impl ParseResult for String {
    fn parse_result(&self) -> GameResult {
        match self.as_str() {
            "Z" => GameResult::Win,
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            _ => panic!("Invalid result"),
        }
    }
}

struct Round {
    pub p1_play: Option<Strategy>,
    pub p2_play: Option<Strategy>,
    pub result: GameResult,
}

impl Round {
    pub fn new(p1_play: impl ParseStrategy, p2_play: impl ParseStrategy, exp_result: impl ParseResult) -> Round {
        Round {
            p1_play: p1_play.parse_strategy(),
            p2_play: p2_play.parse_strategy(),
            result: exp_result.parse_result(),
        }
    }

    pub fn new_from_string(s: &str) -> Round {
        let mut split = s.split_whitespace();
        Round::new(split.next().unwrap().to_string(), None, split.next().unwrap().to_string())
    }

    // Fills in the missing strategy for the round
    // so that player two has the result r
    pub fn fill_none_value(&mut self) {
        match self.result {
            GameResult::Win => {
                if self.p1_play.is_none() {
                    self.p1_play = Some(self.p2_play.as_ref().unwrap().get_winning_strategy());
                }
                if self.p2_play.is_none() {
                    self.p2_play = Some(self.p1_play.as_ref().unwrap().get_losing_strategy());
                }
            }
            GameResult::Lose => {
                if self.p1_play.is_none() {
                    self.p1_play = Some(self.p2_play.as_ref().unwrap().get_losing_strategy());
                }
                if self.p2_play.is_none() {
                    self.p2_play = Some(self.p1_play.as_ref().unwrap().get_winning_strategy());
                }
            }
            GameResult::Draw => {
                if self.p1_play.is_none() {
                    self.p1_play = Some(self.p2_play.as_ref().unwrap().get_draw_strategy());
                }
                if self.p2_play.is_none() {
                    self.p2_play = Some(self.p1_play.as_ref().unwrap().get_draw_strategy());
                }
            }
        }

    }

    fn get_play_score(&self, play: &Strategy) -> i32 {
        match play {
            Strategy::Rock => 1,
            Strategy::Paper => 2,
            Strategy::Scissors => 3,
        }
    }

    pub fn get_p2_score(&self) -> i32 {
        let mut score = 0;

        score += self.get_play_score(&self.p2_play.as_ref().unwrap());

        // Draw
        if self.p2_play.as_ref().unwrap().draws(&self.p1_play.as_ref().unwrap()) {
            score += 3;
        }

        // Wins
        else if self.p2_play.as_ref().unwrap().beats(&self.p1_play.as_ref().unwrap()) {
            score += 6;
        }

        // Loses
        else if self.p2_play.as_ref().unwrap().loses(&self.p1_play.as_ref().unwrap()) {
            score += 0;
        }

        return score;
    }
}

trait Beats {
    fn beats(&self, other: &Strategy) -> bool;
}

trait Draws {
    fn draws(&self, other: &Strategy) -> bool;
}

trait Loses {
    fn loses(&self, other: &Strategy) -> bool;
}

trait StrategyExt {
    fn get_losing_strategy(&self) -> Strategy;
    fn get_winning_strategy(&self) -> Strategy;
    fn get_draw_strategy(&self) -> Strategy;
}

impl Beats for Strategy {
    fn beats(&self, other: &Strategy) -> bool {
        match self {
            Strategy::Rock => match other {
                Strategy::Scissors => true,
                _ => false,
            },
            Strategy::Paper => match other {
                Strategy::Rock => true,
                _ => false,
            },
            Strategy::Scissors => match other {
                Strategy::Paper => true,
                _ => false,
            },
        }
    }
}

impl Draws for Strategy {
    fn draws(&self, other: &Strategy) -> bool {
        match self {
            Strategy::Rock => match other {
                Strategy::Rock => true,
                _ => false,
            },
            Strategy::Paper => match other {
                Strategy::Paper => true,
                _ => false,
            },
            Strategy::Scissors => match other {
                Strategy::Scissors => true,
                _ => false,
            },
        }
    }
}

impl Loses for Strategy {
    fn loses(&self, other: &Strategy) -> bool {
        match self {
            Strategy::Rock => match other {
                Strategy::Paper => true,
                _ => false,
            },
            Strategy::Paper => match other {
                Strategy::Scissors => true,
                _ => false,
            },
            Strategy::Scissors => match other {
                Strategy::Rock => true,
                _ => false,
            },
        }
    }
}

impl StrategyExt for Strategy {
    fn get_losing_strategy(&self) -> Strategy {
        match self {
            Strategy::Rock => Strategy::Paper,
            Strategy::Paper => Strategy::Scissors,
            Strategy::Scissors => Strategy::Rock,
        }
    }

    fn get_winning_strategy(&self) -> Strategy {
        match self {
            Strategy::Rock => Strategy::Scissors,
            Strategy::Paper => Strategy::Rock,
            Strategy::Scissors => Strategy::Paper,
        }
    }

    fn get_draw_strategy(&self) -> Strategy {
        match self {
            Strategy::Rock => Strategy::Rock,
            Strategy::Paper => Strategy::Paper,
            Strategy::Scissors => Strategy::Scissors,
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

    let round_vec = input_str
        .lines()
        .map(|line| Round::new_from_string(line))
        .collect::<Vec<Round>>();

    let mut total_p2_score = 0;
    for mut round in round_vec {
        round.fill_none_value();

        if debug {
            println!("P1: {}, P2: {}", round.p1_play.as_ref().unwrap(), round.p2_play.as_ref().unwrap());
            println!("P2 score: {}", round.get_p2_score());
            println!("----------------");
        }
        total_p2_score += round.get_p2_score();
    }

    println!("Total Score: {}", total_p2_score);
}
