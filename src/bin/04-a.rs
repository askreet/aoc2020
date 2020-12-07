use aoc2020::aoc_04::*;
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let passports = parse_passports(reader);

    let valid_passports = passports.iter().filter(|p| p.is_somewhat_valid()).count();

    println!("Total passports: {}", passports.len());
    println!("Valid passports: {}", valid_passports);
}
