use aoc2020::aoc_03::{Map, Slope};
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let mut map = Map::parse(reader);

    println!("Journey hit {} trees.", map.journey(Slope { right: 3, down: 1 }));
}

