use core::option::Option;
use core::option::Option::{Some, None};
use array2d::Array2D;
use std::io::BufRead;
use core::default::Default;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MapLoc {
    Empty,
    Tree,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Slope {
    pub right: usize,
    pub down: usize,
}

impl MapLoc {
    fn from_char(c: char) -> Option<MapLoc> {
        match c {
            '.' => Some(MapLoc::Empty),
            '#' => Some(MapLoc::Tree),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Map {
    rows: Array2D<MapLoc>,
    width: usize,
    pos: Pos,
}

impl Map {
    #[cfg(test)]
    pub fn from_str(str: &str) -> Map {
        let buf = std::io::Cursor::new(str);

        Map::parse(buf)
    }

    pub fn parse<R: BufRead>(reader: R) -> Map {
        let mut rows_arr: Vec<Vec<MapLoc>> = vec![];

        for line in reader.lines().flatten() {
            rows_arr.push(line.chars().map(|c|
                MapLoc::from_char(c)
                    .unwrap_or_else(|| panic!("Unexpected character: {}", c))).collect());
        }

        Map {
            rows: Array2D::from_rows(&rows_arr),
            width: rows_arr.get(0).unwrap().len(),
            pos: Default::default(),
        }
    }

    pub fn peek(&self) -> &MapLoc {
        self.rows.get(self.pos.row, self.pos.col).unwrap()
    }

    pub fn slide(&mut self, slope: Slope) -> bool {
        if self.pos.row + slope.down > self.rows.num_rows() - 1 {
            false
        } else {
            self.pos.row += slope.down;
            self.pos.col = (self.pos.col + slope.right) % self.width;

            true
        }
    }

    pub fn journey(&mut self, slope: Slope) -> usize {
        self.pos = Pos::default();

        let mut trees_hit: usize = 0;

        loop {
            if !self.slide(slope) {
                return trees_hit;
            }

            if let MapLoc::Tree = self.peek() {
                trees_hit += 1
            }
        }
    }
}

#[test]
fn test_parse_map() {
    let map = Map::from_str("..#\n.#.\n##.");

    use MapLoc::*;
    assert_eq!(
        Map {
            rows: Array2D::from_rows(&[
                vec![Empty, Empty, Tree],
                vec![Empty, Tree, Empty],
                vec![Tree, Tree, Empty],
            ]),
            width: 3,
            pos: Pos { row: 0, col: 0 },
        }
        , map);
}

#[test]
fn test_peek_and_slide() {
    let tests = [
        (2, Slope { right: 1, down: 1 }),
        (7, Slope { right: 3, down: 1 }),
        (3, Slope { right: 5, down: 1 }),
        (4, Slope { right: 7, down: 1 }),
        (2, Slope { right: 1, down: 2 }),
    ];

    for (expected, slope) in tests.iter() {
        let mut map: Map = Map::from_str("..##.......\n\
                                      #...#...#..\n\
                                      .#....#..#.\n\
                                      ..#.#...#.#\n\
                                      .#...##..#.\n\
                                      ..#.##.....\n\
                                      .#.#.#....#\n\
                                      .#........#\n\
                                      #.##...#...\n\
                                      #...##....#\n\
                                      .#..#...#.#");

        assert_eq!(*expected, map.journey(*slope));
    }
}

