extern crate enum_from_str;
#[macro_use] extern crate maplit;

use std::io::{BufReader, BufRead};
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

pub struct RecordReader<T: BufRead> {
    reader: T,
}

impl<T: BufRead> RecordReader<T> {
    pub fn new(reader: T) -> RecordReader<T> {
        RecordReader { reader }
    }
}

impl<T: BufRead> Iterator for RecordReader<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();

        loop {
            match self.reader.read_line(&mut buf) {
                Ok(bytes) =>
                    match (bytes, buf.is_empty()) {
                        (0, true) => return None, // EOF, no pending record.
                        (0, false) => return Some(buf), // EOF, pending record.
                        (1, _) => return Some(buf), // Blank line = end of record.
                        _ => {}
                    },
                Err(e) => panic!(e),
            }
        }
    }
}

pub mod aoc_03;
pub mod aoc_04;
pub mod aoc_05;
pub mod aoc_06;
pub mod aoc_07;
