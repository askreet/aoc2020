use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let filename = std::env::args().nth(1).expect("Must pass filename.");
    let file = File::open(filename).expect("Could not open file.");
    let mut reader = BufReader::new(file);

    let numbers: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();

    for set in numbers.into_iter().combinations(3) {
        if set[0] + set[1] + set[2] == 2020 {
            println!("{:?} = {}", set, set[0] * set[1] * set[2]);
        }
    }
}
