use core::fmt;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use crate::chance::{Chance, WeightedResult};
use crate::config::{Config, Decision};
use crate::constants::{CARD_REGEX, HARD_DRAW_LIMIT, SOFT_DRAW_LIMIT};

#[derive(PartialEq, Clone, Copy)]
pub enum Card { Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten }

impl Card {
    pub fn iterator() -> Iter<'static, Card> {
        static DIRECTIONS: [Card; 10] = [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten];
        DIRECTIONS.iter()
    }

    pub fn value(&self) -> i32 {
        match self {
            Card::Ace => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Card::Ace => "A",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "10"
        })?;
        Ok(())
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn empty() -> Hand {
        Hand { cards: vec![] }
    }

    pub fn parse(s: &str) -> Result<Hand, String> {
        let mut hand = Hand::empty();
        for m in CARD_REGEX.find_iter(s) {
            hand.cards.push(match m.as_str() {
                "1" | "11" | "A" | "a" => Card::Ace,
                "2" => Card::Two,
                "3" => Card::Three,
                "4" => Card::Four,
                "5" => Card::Five,
                "6" => Card::Six,
                "7" => Card::Seven,
                "8" => Card::Eight,
                "9" => Card::Nine,
                "10" | "J" | "j" | "Q" | "q" | "K" | "k" | "X" | "x" => Card::Ten,
                _ => return Result::Err(String::from(&*format!("Invalid input '{}'", m.as_str())))
            });
        }

        Result::Ok(hand)
    }

    pub fn get_score(&self) -> i32 {
        let mut sum = 0;
        let mut has_ace = false;
        for c in &self.cards {
            sum += c.value();
            if *c == Card::Ace && !has_ace { has_ace = true; }
        }
        if has_ace && sum <= 11 { sum += 10; }
        sum
    }

    pub fn is_empty(&self) -> bool { self.cards.is_empty() }
    pub fn is_busted(&self) -> bool { self.get_score() > 21 }
    // pub fn is_blackjack(&self) -> bool { self.get_score() == 21 && self.cards.len() == 2 }

    fn get_effective_score(&self) -> i32 {
        let score = self.get_score();
        if score > 21 {
            -100
        } else if score == 21 && self.cards.len() == 2 {
            100
        } else { score }
    }
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        Hand { cards: self.cards.to_vec() }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for c in &self.cards {
            if first {
                first = false;
            } else {
                f.write_str(" ")?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

pub struct CalculationResult {
    pub chance_when_hit: Chance,
    pub chance_when_stand: Chance,
}

struct Deck {
    cards: [i32; 10],
}

impl Deck {
    // initial 52-card deck
    pub fn new() -> Deck {
        Deck { cards: [4, 4, 4, 4, 4, 4, 4, 4, 4, 16] }
    }

    pub fn iter(&self) -> Iter<'_, i32> { self.cards.iter() }
    // pub fn iter_mut(&mut self) -> IterMut<'_, i32> { self.cards.iter_mut() }
}

impl Index<Card> for Deck {
    type Output = i32;
    fn index(&self, index: Card) -> &Self::Output { &self.cards[(index.value() - 1) as usize] }
}

impl IndexMut<Card> for Deck {
    fn index_mut(&mut self, index: Card) -> &mut Self::Output { &mut self.cards[(index.value() - 1) as usize] }
}

pub fn calculate(dealer: &Hand, player: &Hand, config: &Config) -> Result<CalculationResult, &'static str> {
    if player.is_empty() || dealer.is_empty() { return Result::Err("Fill the hands you dumbass"); }
    if player.is_busted() { return Result::Err("you already lost"); }

    let mut deck = Deck::new();
    assert_eq!(deck.iter().sum::<i32>(), 52);

    for &c in &dealer.cards { deck[c] -= 1; }
    for &c in &player.cards { deck[c] -= 1; }

    for &i in deck.iter() {
        if i < 0 { return Result::Err("Impossible hand, cannot simulate card draw"); }
    }

    fn calculate_stand_chance(deck: &mut Deck, dealer: &mut Hand, player: &Hand, config: &Config) -> Chance {
        if dealer.get_score() <= (if config.soft_17 { SOFT_DRAW_LIMIT } else { HARD_DRAW_LIMIT }) { // draw more
            let mut c: Vec<WeightedResult> = vec![];
            for &card in Card::iterator() {
                let o = deck[card];
                if o <= 0 { continue; }
                deck[card] -= 1;

                dealer.cards.push(card);
                c.push(WeightedResult::of(o, calculate_stand_chance(deck, dealer, player, config)));
                dealer.cards.pop();

                deck[card] = o;
            }
            Chance::from_weighted_result(&c)
        } else {
            match dealer.get_effective_score().cmp(&player.get_effective_score()) {
                Ordering::Less => Chance::win(),
                Ordering::Equal => if player.is_busted() { Chance::lose() } else { Chance::draw() },
                Ordering::Greater => Chance::lose()
            }
        }
    }

    fn calculate_hit_chance(deck: &mut Deck, dealer: &mut Hand, player: &mut Hand, config: &Config) -> Chance {
        let mut c: Vec<WeightedResult> = vec![];
        for &card in Card::iterator() {
            let o = deck[card];
            if o <= 0 { continue; }
            deck[card] -= 1;

            player.cards.push(card);
            c.push(WeightedResult::of(o, if player.is_busted() {
                Chance::lose()
            } else {
                let stand = calculate_stand_chance(deck, dealer, player, config);
                let hit = calculate_hit_chance(deck, dealer, player, config);
                if match config.decision {
                    Decision::MostWin => stand.win > hit.win,
                    Decision::LeastLoss => stand.loss < hit.loss
                } { stand } else { hit }
            }));
            player.cards.pop();

            deck[card] = o;
        }
        Chance::from_weighted_result(&c)
    }

    let mut dealer = dealer.clone();
    let mut player = player.clone();

    Result::Ok(CalculationResult {
        chance_when_hit: calculate_hit_chance(&mut deck, &mut dealer, &mut player, config),
        chance_when_stand: calculate_stand_chance(&mut deck, &mut dealer, &mut player, config),
    })
}