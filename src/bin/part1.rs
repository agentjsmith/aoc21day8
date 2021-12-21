#![feature(int_abs_diff)]

use itertools::Itertools;
use std::{env, fs::File, io::Read};

struct SevenSegDecoder {}

impl SevenSegDecoder {
    fn decode(str: &str) -> Option<u8> {
        // easy digits
        match str.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Puzzle<'a> {
    outputs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(line: &'a str) -> Puzzle<'a> {
        let (_ins, outs) = line.split('|').collect_tuple().expect("missing pipe");
        let outputs = outs.split_whitespace().collect_vec();

        Puzzle { outputs }
    }

    // count how many 1, 4, 7, 8 in the outputs
    fn solve(&self) -> usize {
        let mut total: usize = 0;
        for code in &self.outputs {
            if let Some(_) = SevenSegDecoder::decode(code) {
                total += 1;
            }
        }

        total
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputfile = &args[1];

    let mut fh = File::open(inputfile).expect("Could not open the input file");

    let mut contents = String::new();
    fh.read_to_string(&mut contents)
        .expect("Could not read the input file");

    let items = contents.lines();

    let puzzles: Vec<Puzzle> = items.map(|line| Puzzle::new(line)).collect();

    let total: usize = puzzles.iter().map(|puz| puz.solve()).sum();

    println!("{} total 1, 4, 7, 8s found ", total);
}
