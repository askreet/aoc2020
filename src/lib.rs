extern crate enum_from_str;

use std::io::BufReader;
use std::fs::File;

pub fn create_input_reader() -> BufReader<File> {
    let filename = std::env::args().nth(1).expect("Must pass filename.");
    let file = File::open(filename).expect("Could not open file.");

    BufReader::new(file)
}

macro_rules! regex_captures {
    ($pattern: literal, $input: expr) => {
        {
            lazy_static! {
                static ref THE_REGEX: Regex = Regex::new($pattern).unwrap();
            }

            THE_REGEX.captures($input)
        }
    };
}

pub mod aoc_03;
pub mod aoc_04;

