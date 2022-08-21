extern crate core;

use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

use crate::calc::{calculate, CalculationResult, Hand};
use crate::chance::Chance;
use crate::config::{Config, Decision};
use crate::constants::{CALCULATE_REGEX, CLEAR_REGEX, SET_DECISION_REGEX, SET_HAND_REGEX, SET_SOFT_17_REGEX, VERSION};

mod calc;
mod chance;
mod constants;
mod config;

fn main() {
    let mut dealer = Hand::empty();
    let mut player = Hand::empty();
    let mut calculation_result: Option<(CalculationResult, Duration)> = Option::None;
    let mut config = Config {
        soft_17: true,
        decision: Decision::MostWin,
    };

    loop {
        println!("BLACKJACK CRACKER v{}", VERSION.unwrap_or("???"));
        println!();
        println!("COMMANDS");
        println!("> D: 1 2 3 4... => Set dealer hand");
        println!("> P: 1 2 3 4... => Set player hand");
        println!("> Calc          => Calculate, duh");
        println!("> Decision = N  => Set decision factor; 1: Most win, 2: Least loss");
        println!("> Soft17   = N  => Set soft 17; Y: Soft 17, N: Hard 17");
        println!("> Clear         => Clear the hand");
        println!();
        println!();

        // tell hands

        if !dealer.is_empty() {
            if dealer.is_busted() {
                println!("DEALER: {} (BUST!)", dealer);
            } else {
                println!("DEALER: {} ({})", dealer, dealer.get_score());
            }
        }
        if !player.is_empty() {
            if player.is_busted() {
                println!("PLAYER: {} (BUST!)", player);
            } else {
                println!("PLAYER: {} ({})", player, player.get_score());
            }
        }
        if !dealer.is_empty() || !player.is_empty() { println!(); }

        if dealer.is_empty() || player.is_empty() {
            println!("Set hand to calculate things");
        } else {
            if let Some((res, dur)) = &calculation_result {
                fn print(chance: &Chance) {
                    println!("    WIN chance  : {:.2}%", chance.win * 100.0);
                    println!("    LOSE chance : {:.2}%", chance.loss * 100.0);
                    println!("    DRAW chance : {:.2}%", chance.draw * 100.0);
                }
                println!("WHEN HIT:");
                print(&res.chance_when_hit);
                println!();
                println!("WHEN STAND:");
                print(&res.chance_when_stand);
                println!();
                println!("Calculated in {:?}", dur);
            }
            println!();
            println!("Decision: {:?}", config.decision);
            println!("Soft 17: {:?}", config.soft_17);
        }

        println!();


        loop {
            print!("> ");
            stdout().flush().expect("oh no");
            let mut s = String::new();
            stdin().read_line(&mut s).expect("wtf dude");
            let s = &*s;
            if let Some(captures) = SET_HAND_REGEX.captures(s) {
                if let Ok(new_hand) = Hand::parse(captures.get(2).unwrap().as_str()) {
                    let hand: &mut Hand = match captures.get(1).unwrap().as_str() {
                        "D" | "d" => &mut dealer,
                        "P" | "p" => &mut player,
                        _ => {
                            println!("Unexpected error");
                            continue;
                        }
                    };
                    *hand = new_hand;
                } else {
                    println!("Unexpected error");
                    continue;
                }
                calculation_result = Option::None;
            } else if let Some(captures) = SET_DECISION_REGEX.captures(s) {
                match captures.get(1).unwrap().as_str() {
                    "1" => config.decision = Decision::MostWin,
                    "2" => config.decision = Decision::LeastLoss,
                    _ => {
                        println!("Unexpected error");
                        continue;
                    }
                }
                calculation_result = Option::None;
            } else if let Some(captures) = SET_SOFT_17_REGEX.captures(s) {
                match captures.get(1).unwrap().as_str() {
                    "Y" | "y" | "O" | "o" | "T" | "t" => config.soft_17 = true,
                    "N" | "n" | "X" | "x" | "F" | "f" => config.soft_17 = false,
                    _ => {
                        println!("Unexpected error");
                        continue;
                    }
                }
                calculation_result = Option::None;
            } else if CALCULATE_REGEX.is_match(s) {
                let start = Instant::now();
                match calculate(&dealer, &player, &config) {
                    Ok(result) => calculation_result = Option::Some((result, start.elapsed())),
                    Err(t) => {
                        println!("Calculation failed: {}", t);
                        continue;
                    }
                }
            } else if CLEAR_REGEX.is_match(s) {
                dealer.cards.clear();
                player.cards.clear();
                calculation_result = Option::None;
            } else {
                println!("Invalid input");
                continue;
            }
            println!();
            println!();
            println!();
            break;
        }
    }
}