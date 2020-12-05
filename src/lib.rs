use std::io::BufReader;
use std::fs::File;

pub mod aoc_03;

pub fn create_input_reader() -> BufReader<File> {
    let filename = std::env::args().nth(1).expect("Must pass filename.");
    let file = File::open(filename).expect("Could not open file.");

    BufReader::new(file)
}
