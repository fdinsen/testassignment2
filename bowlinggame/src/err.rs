#[derive(Debug, Clone)]
pub enum BowlingErr {
    NoRollsLeft,
    TooManyPins(u8),
}
#[derive(Debug, Clone)]
struct NoRollsLeftError;

impl std::fmt::Display for BowlingErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BowlingErr::NoRollsLeft => write!(f, "No rolls left in frame."),
            BowlingErr::TooManyPins(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for BowlingErr {}