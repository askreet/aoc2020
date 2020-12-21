#[derive(Clone, PartialEq, Eq, Debug)]
struct SeatRange {
    lower: u32,
    upper: u32,
}

impl SeatRange {
    fn keep_lower_half(&mut self) {
        assert!(self.len() > 1);

        self.upper -= self.len() / 2;
    }

    fn keep_upper_half(&mut self) {
        assert!(self.len() > 1);

        self.lower += self.len() / 2;
    }

    fn len(&self) -> u32 {
        (self.upper + 1) - self.lower
    }

    fn resolve(&self) -> Option<u32> {
        if self.lower == self.upper {
            Some(self.lower)
        } else {
            None
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Seat {
    pub row: u32,
    pub col: u32,
}

impl Seat {
    pub fn at(row: u32, col: u32) -> Seat {
        Seat { row, col }
    }

    pub fn from_ticket(ticket: &str) -> Seat {
        let mut rows = SeatRange { lower: 0, upper: 127 };
        let mut cols = SeatRange { lower: 0, upper: 7 };

        ticket.chars().for_each(|c| {
            match c {
                'F' => rows.keep_lower_half(),
                'B' => rows.keep_upper_half(),
                'R' => cols.keep_upper_half(),
                'L' => cols.keep_lower_half(),
                _ => panic!("Unexpected input character: {}", c),
            }
        });

        Seat {
            row: rows.resolve().expect("Rows not sufficiently resolved."),
            col: cols.resolve().expect("Cols not sufficiently resolved."),
        }
    }

    pub fn id(&self) -> u32 {
        (self.row * 8) + self.col
    }
}

#[test]
fn test_id() {
    assert_eq!(Seat::at(44, 5).id(), 357);
    assert_eq!(Seat::at(70, 7).id(), 567);
    assert_eq!(Seat::at(14, 7).id(), 119);
    assert_eq!(Seat::at(102, 4).id(), 820);
}

#[test]
fn test_from_ticket() {
    assert_eq!(Seat::from_ticket("FBFBBFFRLR"), Seat { row: 44, col: 5 });
    assert_eq!(Seat::from_ticket("BFFFBBFRRR"), Seat { row: 70, col: 7 });
    assert_eq!(Seat::from_ticket("FFFBBBFRRR"), Seat { row: 14, col: 7 });
    assert_eq!(Seat::from_ticket("BBFFBBFRLL"), Seat { row: 102, col: 4 });
}
