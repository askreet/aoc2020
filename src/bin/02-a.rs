use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Regex, Captures};

#[derive(Debug, PartialEq, Eq)]
struct PasswordPolicy {
    letter: char,
    minimum: u8,
    maximum: u8,
}

impl PasswordPolicy {
    fn check(&self, input: &str) -> bool {
        let mut count: u8 = 0;
        for char in input.chars() {
            if char == self.letter {
                count = count + 1;
            }
        }

        (self.minimum..=self.maximum).contains(&count)
    }
}

#[test]
fn test_conforming_passwords() {
    let tests = vec![
        (PasswordPolicy { letter: 'a', minimum: 1, maximum: 3 }, "abcde"),
        (PasswordPolicy { letter: 'c', minimum: 2, maximum: 9 }, "ccccccccc"),
    ];

    for (policy, password) in tests {
        assert!(policy.check(password));
    }
}

#[test]
fn test_non_conforming_passwords() {
    let tests = vec![
        (PasswordPolicy { letter: 'b', minimum: 1, maximum: 3 }, "cdefg"),
    ];

    for (policy, password) in tests {
        assert!(false == policy.check(password));
    }
}

fn parse(line: &str) -> (PasswordPolicy, String) {
    let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();

    let captures = re.captures(line);

    match captures {
        None => panic!("Line does not match expected pattern: {}", line),
        Some(captures) => {
            (
                PasswordPolicy {
                    letter: captures[3].chars().nth(0).unwrap(),
                    minimum: captures[1].parse::<u8>().unwrap(),
                    maximum: captures[2].parse::<u8>().unwrap(),
                },
                captures[4].into()
            )
        }
    }
}

#[test]
fn test_parser() {
    let (policy, password) = parse("2-12 d: abcdef");

    assert_eq!(
        PasswordPolicy {
            letter: 'd',
            minimum: 2,
            maximum: 12,
        },
        policy
    );

    assert_eq!("abcdef", password);
}

fn main() {
    let filename = std::env::args().nth(1).expect("Must pass filename.");
    let file = File::open(filename).expect("Could not open file.");
    let mut reader = BufReader::new(file);

    let mut count_valid = 0;

    for ref line in reader.lines().flatten() {
        let (policy, password) = parse(line);

        let valid = policy.check(password.as_ref());

        if valid {
            count_valid = count_valid + 1;
        }

        println!("{}: {}", line, valid);
    }

    println!("Total: {}", count_valid);
}
