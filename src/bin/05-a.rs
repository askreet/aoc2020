use aoc2020::aoc_05::*;
use aoc2020::create_input_reader;
use std::io::BufRead;

fn main() {
    let reader = create_input_reader();

    let seats: Vec<Seat> = reader.lines()
        .map(|l| Seat::from_ticket(l.unwrap().trim()))
        .collect();

    let highest_seat_id = seats.iter().map(|s| s.id()).max().unwrap();

    println!("Total seats: {}", seats.len());
    println!("Max ID: {}", highest_seat_id);
}
