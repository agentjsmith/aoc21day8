#![feature(let_else)]

pub mod digit;
pub mod segment;
pub mod sevensegdecoder;

use itertools::Itertools;
use std::{env, fs::File, io::Read};

#[derive(Debug)]
struct Puzzle<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(line: &'a str) -> Puzzle<'a> {
        let (ins, outs) = line
            .split('|')
            .collect_tuple()
            .expect("input line missing pipe");
        let inputs = ins.split_whitespace().collect_vec();
        let outputs = outs.split_whitespace().collect_vec();

        Puzzle { inputs, outputs }
    }

    fn solve(&self) -> usize {
        let ssd = sevensegdecoder::SevenSegDecoder::new(&self.inputs);
        let digits: Vec<u8> = self.outputs.iter().filter_map(|d| ssd.decode(d)).collect();
        digits.iter().fold(0, |acc, &dig| acc * 10 + dig as usize)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // This program takes one required parameter: the name of a file containing input
    let Some(input_file) = &args.get(1) else {
        println!("Hey!  An argument is required.");
        println!("Usage: {} input.txt",&args[0]);
        std::process::exit(1);
    };

    let mut fh = File::open(input_file).expect("Could not open the input file");

    let mut contents = String::new();
    fh.read_to_string(&mut contents)
        .expect("Could not read the input file");

    let puzzles = contents.lines().map(|line| Puzzle::new(line));

    let total: usize = puzzles.map(|puz| puz.solve()).sum();

    println!("{} total", total);
}
