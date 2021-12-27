use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Digit {
    Decided(u8),
    Undecided(HashSet<u8>),
    Invalid,
}

impl Digit {
    pub fn is_decided(&self) -> bool {
        use Digit::*;
        match self {
            Decided(_) => true,
            Undecided(_) => false,
            Invalid => false,
        }
    }
}