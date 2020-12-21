use aoc2020::aoc_05::*;
use aoc2020::create_input_reader;
use aoc2020::RecordReader;
use std::io::BufRead;
use std::cmp::Ordering;
use aoc2020::aoc_06::unique_chars;

fn main() {
    let reader = RecordReader::new(create_input_reader());

    let result = reader.fold(0, |a, e| a + unique_chars(e.as_str()).len());

    println!("Result = {}", result);
}
