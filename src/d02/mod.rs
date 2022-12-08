use std::{io::{BufReader, BufRead, Read}};

const MY_ROCK: char = 'X';
const MY_PAPER: char = 'Y';
const MY_SCISSORS: char = 'Z';

const LOST_SYMBOL: char = 'X';
const DRAW_SYMBOL: char = 'Y';
const WON_SYMBOL: char = 'Z';

const OPP_ROCK: char = 'A';
const OPP_PAPER: char = 'B';
const OPP_SCISSORS: char = 'C';

const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

fn symbol_score(symbol: char) -> Result<u32, String> {
    match symbol {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err(format!("Invalid symbol {}", symbol))
    }
}

fn match_score(opponent: char, mine: char) -> Result<u32, String> {
    match (opponent, mine) {
        (OPP_ROCK, MY_ROCK) => Ok(DRAW),
        (OPP_ROCK, MY_PAPER) => Ok(WON),
        (OPP_ROCK, MY_SCISSORS) => Ok(LOST),

        (OPP_PAPER, MY_ROCK) => Ok(LOST),
        (OPP_PAPER, MY_PAPER) => Ok(DRAW),
        (OPP_PAPER, MY_SCISSORS) => Ok(WON),

        (OPP_SCISSORS, MY_ROCK) => Ok(WON),
        (OPP_SCISSORS, MY_PAPER) => Ok(LOST),
        (OPP_SCISSORS, MY_SCISSORS) => Ok(DRAW),
        _ => Err(format!("Invalid round: {} {}", opponent, mine)),
    }
}

fn select_symbol(opponent: char, result: char) -> Result<char, String> {
    match (opponent, result) {
        (OPP_ROCK, DRAW_SYMBOL) => Ok(MY_ROCK),
        (OPP_ROCK, WON_SYMBOL) => Ok(MY_PAPER),
        (OPP_ROCK, LOST_SYMBOL) => Ok(MY_SCISSORS),

        (OPP_PAPER, DRAW_SYMBOL) => Ok(MY_PAPER),
        (OPP_PAPER, WON_SYMBOL) => Ok(MY_SCISSORS),
        (OPP_PAPER, LOST_SYMBOL) => Ok(MY_ROCK),

        (OPP_SCISSORS, DRAW_SYMBOL) => Ok(MY_SCISSORS),
        (OPP_SCISSORS, WON_SYMBOL) => Ok(MY_ROCK),
        (OPP_SCISSORS, LOST_SYMBOL) => Ok(MY_PAPER),
        _ => Err(format!("Invalid round: {} {}", opponent, result)),
    }
}

pub fn run<R: Read>(input: BufReader<R>) -> Result<(String, String), String> {
    let mut total1 = 0u32;
    let mut total2 = 0u32;
    for line in input.lines() {
        match line {
            Ok(text) => {
                if text.len() == 0 {
                    continue;
                }
                let mut chars = text.chars();
                let first_symbol = chars.nth(0).unwrap(); // This consumes element from iterator
                let second_symbol = chars.nth(1).unwrap();

                let score1 = symbol_score(second_symbol).unwrap() + match_score(first_symbol, second_symbol).unwrap();
                total1 += score1;

                let mine_select = select_symbol(first_symbol, second_symbol).unwrap();
                let score2 = symbol_score(mine_select).unwrap() + match_score(first_symbol, mine_select).unwrap();
                total2 += score2;
            },
            Err(e) => return Err(e.to_string())
        }
    }
    return Ok((total1.to_string(), total2.to_string()));
}
