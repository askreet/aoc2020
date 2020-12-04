use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = std::env::args().nth(1).expect("Must pass filename.");
    let file = File::open(filename).expect("Could not open file.");
    let mut reader = BufReader::new(file);

    let numbers: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    let mut number_exists: [bool; 2021] = [false; 2021];

    for value in &numbers {
        if *value > 2020 {
            panic!("Invalid input value: {}", value);
        }

        number_exists[*value as usize] = true;
    }

    for value in &numbers {
        let inverse_value = 2020 - value;

        if number_exists.get(inverse_value as usize) == Some(&true) {
            println!("Value {} has inverse value {}. Result is {}", value,
                     inverse_value, value * inverse_value);
            return;
        }
    }
}
