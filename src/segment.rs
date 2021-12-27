use std::collections::HashSet;

#[derive(Debug)]
pub struct Segment {
    pub candidates: HashSet<char>,
}

impl Segment {
    pub fn new() -> Segment {
        Segment {
            candidates: ['A', 'B', 'C', 'D', 'E', 'F', 'G'].into(),
        }
    }

    pub fn eliminate(&mut self, candidate: &char) {
        self.candidates.remove(candidate);
    }
}
