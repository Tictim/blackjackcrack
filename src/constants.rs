use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref SET_HAND_REGEX: Regex = RegexBuilder::new(r"\s*([DP])\s*:\s*((?:(?:10|11|1|2|3|4|5|6|7|8|9|A|K|Q|J|a|k|q|j|X|x)(?:\s+|\s*,?\s*))*)$")
        .case_insensitive(true)
        .build().unwrap();

    pub static ref CALCULATE_REGEX: Regex = RegexBuilder::new(r"\s*calc\s*$")
        .case_insensitive(true)
        .build().unwrap();

    pub static ref SET_DECISION_REGEX: Regex = RegexBuilder::new(r"\s*decision\s*=\s*([123])\s*$")
        .case_insensitive(true)
        .build().unwrap();

    pub static ref SET_SOFT_17_REGEX: Regex = RegexBuilder::new(r"\s*soft\s*17\s*=\s*([yYnNtTfFoOxX])\s*$")
        .case_insensitive(true)
        .build().unwrap();

    pub static ref CLEAR_REGEX: Regex = RegexBuilder::new(r"\s*clear\s*$")
        .case_insensitive(true)
        .build().unwrap();

    pub static ref CARD_REGEX: Regex = Regex::new(r"10|11|1|2|3|4|5|6|7|8|9|A|K|Q|J|a|k|q|j|X|x").unwrap();
}

pub const SOFT_DRAW_LIMIT: i32 = 16; // Draws UNTIL EXCEEDING THIS NUMBER
pub const HARD_DRAW_LIMIT: i32 = 17; // Draws UNTIL EXCEEDING THIS NUMBER