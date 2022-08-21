pub struct Config {
    pub soft_17: bool,
    pub decision: Decision
}

#[derive(Clone, Copy, Debug)]
pub enum Decision { MostWin, LeastLoss }
