use super::digit::Digit;
use super::segment::Segment;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct SevenSegDecoder {
    digits: HashMap<String, Digit>,
}

// Convention: lowercase letters are scrambled inputs, capitals are decoded outputs
impl SevenSegDecoder {
    pub fn new(inputs: &Vec<&str>) -> SevenSegDecoder {
        let mut segments: HashMap<char, Segment> = HashMap::new(); // map of input "wire" to possible output "segment"
        let mut digits: HashMap<String, Digit> = HashMap::new(); // map of input strings to possible output digits

        for seg in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            segments.insert(seg, Segment::new());
        }

        // the shorter codes have unique solutions so do them first
        let inp = inputs.iter().sorted_unstable_by_key(|&&k| k.len());

        // First use the digits of known length to rule out as many implausible mappings as possible
        for i in inp {
            let input: HashSet<char> = Self::str_to_hashset(i);

            let dig = Self::lookup_digits(&input);

            if let Digit::Decided(n) = dig {
                Self::update_segments(&input, n, &mut segments);
            }

            //let sr: String = String::from(*i);
            let sr: String = Self::hashset_to_str(&input);
            digits.insert(sr, dig);
        }

        // Then grind through the digits that are still unknown and eliminate implausible possibilities from the list until all are solved
        let cooler_digits = digits.clone();
        let mut undecided: HashSet<&String> = cooler_digits
            .iter()
            .filter_map(|(key, d)| if !d.is_decided() { Some(key) } else { None })
            .collect();
        let mut num_undecided = undecided.len();

        while num_undecided > 0 {
            let mut found: Vec<&String> = Vec::new();
            for cipher_string in &undecided {
                // all combinations of cleartext characters that haven't been ruled out yet
                let Digit::Undecided(plausible_digits) = digits.get(*cipher_string).expect("it's ok") else {
                   panic!("at the disco");
               };
                let plausible_clear_strings: HashSet<String> =
                    Self::all_possibilities(&cipher_string, &segments);
                let valid_digit_strings: Vec<String> = plausible_digits
                    .into_iter()
                    .map(|n| Self::digit_to_segments(*n))
                    .map(|h| Self::hashset_to_str(&h))
                    .collect();

                //print!("{} might be {:?}",cipher_string,plausible_clear_strings);
                //println!(", valid ones are {:?}",valid_digit_strings);

                let mut matching_digit_strings: Vec<String> = Vec::new();
                for dig in valid_digit_strings {
                    if plausible_clear_strings.contains(&dig) {
                        matching_digit_strings.push(dig);
                    }
                }

                // Found a unique digit, claim it!
                if matching_digit_strings.len() == 1 {
                    let clear_string = matching_digit_strings
                        .into_iter()
                        .next()
                        .expect("don't worry");
                    let real_digit = Self::segments_to_digit(&Self::str_to_hashset(&clear_string))
                        .expect("this had better not fail");

                    Self::update_segments(
                        &Self::str_to_hashset(&cipher_string),
                        real_digit,
                        &mut segments,
                    );
                    let key = (&**cipher_string.clone()).to_string();
                    digits.insert(key, Digit::Decided(real_digit));

                    println!("{} is {}!", cipher_string, real_digit);

                    found.push(*cipher_string);
                }
            }

            // avoid endless loop if it's implausible
            if found.is_empty() {
                panic!("forward progress stopped while grinding for digits")
            }

            // remove any that are now decided from the undecided list
            for f in found {
                undecided.remove(&f);
                num_undecided -= 1;
            }
        }

        SevenSegDecoder { digits }
    }

    fn sort_chars(str: &mut String) {
        let mut char_vec: Vec<char> = str.chars().collect();
        char_vec.sort_unstable();
        char_vec.dedup();
        *str = char_vec.into_iter().collect::<String>();
    }

    // Expand a ciphertext into every possible cleartext given the wiring that is known so far
    fn all_possibilities(cipher: &String, segments: &HashMap<char, Segment>) -> HashSet<String> {
        let mut tmpvec: Vec<String> = vec!["".to_string()];

        for cipher_char in cipher.chars() {
            let mut newvec: Vec<String> = Vec::new();
            let candidates: Vec<&char> = segments
                .get(&cipher_char)
                .expect("won't fail")
                .candidates
                .iter()
                .collect();

            for i in tmpvec {
                for j in &candidates {
                    let mut fullstr = i.clone();
                    let tail: String = j.to_string();
                    let tailstr: &str = &tail;
                    fullstr.push_str(tailstr);
                    SevenSegDecoder::sort_chars(&mut fullstr);
                    newvec.push(fullstr);
                }
            }
            tmpvec = newvec;
            tmpvec.sort_unstable();
            tmpvec.dedup();
        }

        // only keep strings that are the same length after deduping; duplicates were never plausible
        let cleanvec = tmpvec.into_iter().filter(|s| s.len() == cipher.len());
        HashSet::from_iter(cleanvec)
    }

    fn update_segments(
        code_segments_on: &HashSet<char>,
        digit: u8,
        segments: &mut HashMap<char, Segment>,
    ) {
        let code_segments_off = SevenSegDecoder::invert_segments(code_segments_on);
        let clear_segments_on = SevenSegDecoder::digit_to_segments(digit);
        let clear_segments_off = SevenSegDecoder::invert_segments(&clear_segments_on);

        // Code characters that are "ON" can not be clear characters that are "OFF"
        for code in code_segments_on {
            let s: &mut Segment = segments.get_mut(&code).expect("done goofed");
            for c in &clear_segments_off {
                s.eliminate(c);
            }
        }

        // Code characters that are "OFF" can not be clear characters that are "ON"
        for code in code_segments_off {
            let s: &mut Segment = segments.get_mut(&code).expect("goofed again");
            for c in &clear_segments_on {
                s.eliminate(c);
            }
        }
    }

    fn invert_segments(input: &HashSet<char>) -> HashSet<char> {
        let uppercase = input.iter().next().unwrap_or(&' ').is_uppercase();

        let all_segments: HashSet<char> = if uppercase {
            ['A', 'B', 'C', 'D', 'E', 'F', 'G'].into()
        } else {
            ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into()
        };

        all_segments.difference(&input).map(|&x| x).collect()
    }

    fn segments_to_digit(hs: &HashSet<char>) -> Option<u8> {
        let s = SevenSegDecoder::hashset_to_str(hs);
        let sr: &str = &s;

        match sr {
            "ABCEFG" => Some(0),
            "CF" => Some(1),
            "ACDEG" => Some(2),
            "ACDFG" => Some(3),
            "BCDF" => Some(4),
            "ABDFG" => Some(5),
            "ABDEFG" => Some(6),
            "ACF" => Some(7),
            "ABCDEFG" => Some(8),
            "ABCDFG" => Some(9),
            _ => None,
        }
    }

    fn str_to_hashset(str: &str) -> HashSet<char> {
        str.chars().collect::<HashSet<char>>()
    }

    fn hashset_to_str(hs: &HashSet<char>) -> String {
        hs.iter().sorted_unstable().collect::<String>()
    }

    fn digit_to_segments(dig: u8) -> HashSet<char> {
        let s = match dig {
            0 => "ABCEFG",
            1 => "CF",
            2 => "ACDEG",
            3 => "ACDFG",
            4 => "BCDF",
            5 => "ABDFG",
            6 => "ABDEFG",
            7 => "ACF",
            8 => "ABCDEFG",
            9 => "ABCDFG",
            _ => panic!("Internal error: invalid digit in digit_to_segments"),
        };

        SevenSegDecoder::str_to_hashset(s)
    }

    fn lookup_digits(code: &HashSet<char>) -> Digit {
        use Digit::*;
        match code.len() {
            2 => Decided(1),
            3 => Decided(7),
            4 => Decided(4),
            5 => Undecided([2, 3, 5].into()),
            6 => Undecided([0, 6, 9].into()),
            7 => Decided(8),
            _ => Invalid,
        }
    }

    pub fn decode(&self, code: &str) -> Option<u8> {
        // roundtrip through conversions to sort the inputs
        let tmp_hs = Self::str_to_hashset(code);
        let sorted_code = Self::hashset_to_str(&tmp_hs);

        if let Some(Digit::Decided(num)) = self.digits.get(&sorted_code) {
            Some(*num)
        } else {
            None
        }
    }
}
