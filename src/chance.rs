use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Chance {
    pub win: f64,
    pub draw: f64,
    pub loss: f64
}

impl Add for Chance {
    type Output = Chance;
    fn add(self, rhs: Self) -> Self::Output {
        Chance::new(self.win + rhs.win, self.draw + rhs.draw, self.loss + rhs.loss)
    }
}

impl AddAssign for Chance {
    fn add_assign(&mut self, rhs: Self) {
        self.win += rhs.win;
        self.draw += rhs.draw;
        self.loss += rhs.loss;
    }
}

impl Sum for Chance {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut chance = Chance::new(0.0, 0.0, 0.0);
        for c in iter { chance += c; }
        chance
    }
}

impl Div<f64> for Chance {
    type Output = Chance;
    fn div(self, rhs: f64) -> Self::Output {
        Chance::new(self.win / rhs, self.draw / rhs, self.loss / rhs)
    }
}

impl Mul<f64> for Chance {
    type Output = Chance;
    fn mul(self, rhs: f64) -> Self::Output {
        Chance::new(self.win * rhs, self.draw * rhs, self.loss * rhs)
    }
}

impl Chance {
    pub fn new(win: f64, draw: f64, loss: f64) -> Chance {
        Chance { win, draw, loss }
    }

    pub fn win() -> Chance { Chance::new(1.0, 0.0, 0.0) }
    pub fn draw() -> Chance { Chance::new(0.0, 1.0, 0.0) }
    pub fn lose() -> Chance { Chance::new(0.0, 0.0, 1.0) }

    pub fn from_weighted_result(results: &Vec<WeightedResult>) -> Chance {
        let wgt: i32 = results.iter().map(|r| -> i32{ r.weight }).sum();
        let chance = results.iter().map(|r| -> Chance { r.result*r.weight as f64 }).sum::<Chance>();
        chance / wgt as f64
    }
}

pub struct WeightedResult {
    pub weight: i32,
    pub result: Chance,
}

impl WeightedResult {
    pub fn of(weight: i32, result: Chance) -> WeightedResult {
        WeightedResult { weight, result }
    }
}
