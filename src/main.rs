#![feature(let_else)]

pub mod digit;
pub mod segment;
pub mod sevensegdecoder;

use itertools::Itertools;
use regex::Regex;
use std::{env, fs::File, io::Read};

#[derive(Debug)]
struct Puzzle<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(line: &'a str) -> Option<Puzzle<'a>> {
        if let Some((ins, outs)) = line.split('|').collect_tuple() {
            let inputs = ins.split_whitespace().collect_vec();
            let outputs = outs.split_whitespace().collect_vec();

            // validate input and output strings contain only characters a-g
            let valid_chars = Regex::new(r"^[a-g]+$").unwrap(); // static, valid regex.  won't panic.
            for input in &inputs {
                if !valid_chars.is_match(input) {
                    println!(
                        "Input Error: Invalid character found in input string {}",
                        input
                    );
                    return None;
                }
            }
            for output in &outputs {
                if !valid_chars.is_match(output) {
                    println!(
                        "Input Error: Invalid character found in output string {}",
                        output
                    );
                    return None;
                }
            }

            Some(Puzzle { inputs, outputs })
        } else {
            println!(
                "Input Error: Puzzle string lacks a pipe separator:\n{}",
                line
            );
            None
        }
    }

    fn solve(&self) -> Option<usize> {
        let ssd = sevensegdecoder::SevenSegDecoder::new(&self.inputs);

        // attempt to translate all given output codes...
        let digits: Vec<Option<u8>> = self.outputs.iter().map(|d| ssd.decode(d)).collect();

        // if any failed to decode, fail the entire puzzle
        if digits.iter().any(|d| d.is_none()) {
            println!("Input Error: At least one given output failed to decode");
            println!("     Decoder was {:?}", ssd);
            println!("     Output codes were {:?}", self.outputs);

            None
        } else {
            // unwrap will not panic because we already checked for None
            Some(
                digits
                    .iter()
                    .fold(0, |acc, &dig| acc * 10 + dig.unwrap() as usize),
            )
        }
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

    // The filter_map's cause these loops to ignore any invalid puzzles.  Diagnostics are produced elsewhere.
    let puzzles = contents.lines().filter_map(Puzzle::new);
    let total: usize = puzzles.filter_map(|puz| puz.solve()).sum();

    println!("{} total", total);
}
