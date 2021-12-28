use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Digit {
    Decided(u8),
    Undecided(HashSet<u8>),
    Invalid,
}

impl Digit {
    pub fn is_undecided(&self) -> bool {
        use Digit::*;
        match self {
            Undecided(_) => true,
            Decided(_) => false,
            Invalid => false,
        }
    }
}
