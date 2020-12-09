use aoc2020::aoc_05::*;
use aoc2020::create_input_reader;
use std::io::BufRead;
use std::cmp::Ordering;

fn main() {
    let reader = create_input_reader();

    let mut seats: Vec<Seat> = reader.lines()
        .map(|l| Seat::from_ticket(l.unwrap().trim()))
        .collect();

    seats.sort_by(|a, b|
        if a.id() < b.id() {
            Ordering::Less
        } else {
            Ordering::Greater
        });

    let mut last_id = 0;
    for seat in &seats {
        if last_id == 0 {

        } else if last_id != seat.id() - 1 {
            println!("Found missing seat at ID = {}", seat.id() - 1);
        }

        last_id = seat.id();
    }

    println!("Total seats: {}", seats.len());
}
