use aoc2020::aoc_03::*;
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let mut map = Map::parse(reader);

    let mut slope_product = 1;

    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    for slope in slopes.iter() {
        let trees_hit = map.journey(*slope);
        slope_product *= trees_hit;

        println!("Journey along {:?} hit {} trees.", slope, trees_hit);
    }

    println!("Result: {}", slope_product);
}
