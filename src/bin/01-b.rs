use std::io::BufRead;
use itertools::Itertools;
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let numbers: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();

    for set in numbers.into_iter().combinations(3) {
        if set[0] + set[1] + set[2] == 2020 {
            println!("{:?} = {}", set, set[0] * set[1] * set[2]);
        }
    }
}
