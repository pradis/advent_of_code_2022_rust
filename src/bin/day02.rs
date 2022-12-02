use std::fs;
use std::io::{BufRead, BufReader, Error};


fn player_game(input: char) -> u32 {
    match input {
        'A' | 'X' => { 1 }
        'B' | 'Y' => { 2 }
        'C' | 'Z' => { 3 }
        _ => panic!("crash and burn")
    }
}

struct Round {
    choice_player_1: char,
    choice_player_2: char,
}

impl Round {
    fn init(line: String) -> Round {
        let char_vec: Vec<char> = line.chars().collect();
        Round {
            choice_player_1: char_vec[0],
            choice_player_2: char_vec[2],
        }
    }

    fn result(&self) -> u32 {
        let result_player_2: u32 = player_game(self.choice_player_2);
        match (self.choice_player_1, self.choice_player_2) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => { result_player_2 + 6 }
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => { result_player_2 }
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => { result_player_2 + 3 }
            _ => { panic!("crash and burn") }
        }
    }

    fn strategy_mapping(&self) -> Round {
        let new_value = match (self.choice_player_1, self.choice_player_2) {
            ('A', 'X') => 'Z',
            ('B', 'X') => 'X',
            ('C', 'X') => 'Y',
            ('A', 'Y') => 'X',
            ('B', 'Y') => 'Y',
            ('C', 'Y') => 'Z',
            ('A', 'Z') => 'Y',
            ('B', 'Z') => 'Z',
            ('C', 'Z') => 'X',
            _ => { panic!("crash and burn") }
        };
        Round {
            choice_player_1: self.choice_player_1,
            choice_player_2: new_value,
        }
    }
}


fn ex1(path: &str) -> u32 {
    let file = fs::File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let result: u32 = reader.lines()
        .fold(0, |result_games: u32, line| {
            let maybe_line = line.expect("line with some issues");
            result_games + Round::init(maybe_line).result()
        });
    result
}

fn ex2(path: &str) -> u32 {
    let file = fs::File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let result: u32 = reader.lines()
        .fold(0, |result_games: u32, line| {
            let maybe_line = line.expect("line with some issues");
            result_games + Round::init(maybe_line).strategy_mapping().result()
        });
    result
}

fn main() -> Result<(), Error> {
    println!("example es1: {}", ex1("resources/examples/day_02.txt"));
    println!("result es1: {}", ex1("resources/inputs/day_02.txt"));
    println!("example es2: {}", ex2("resources/examples/day_02.txt"));
    println!("result es2: {}", ex2("resources/inputs/day_02.txt"));
    Ok(())
}
