use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::{Regex, Captures};

#[derive(Debug, PartialEq, Eq)]
struct PasswordPolicy {
    letter: char,
    positions: [ usize; 2 ],
}

impl PasswordPolicy {
    fn check(&self, input: &str) -> bool {
        let mut count: u8 = 0;
        for char in input.chars() {
            if char == self.letter {
                count = count + 1;
            }
        }

        (input.chars().nth(self.positions[0] - 1) == Some(self.letter)) ^
            (input.chars().nth(self.positions[1] - 1) == Some(self.letter))
    }
}

#[test]
fn test_conforming_passwords() {
    let tests = vec![
        (PasswordPolicy { letter: 'a', positions: [ 1, 3 ] }, "abcde"),
    ];

    for (policy, password) in tests {
        assert!(policy.check(password));
    }
}

#[test]
fn test_non_conforming_passwords() {
    let tests = vec![
        (PasswordPolicy { letter: 'b', positions: [ 1, 3 ] }, "cdefg"),
        (PasswordPolicy { letter: 'c', positions: [ 2, 9 ] }, "ccccccccc"),
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
                    positions: [
                        captures[1].parse::<usize>().unwrap(),
                        captures[2].parse::<usize>().unwrap(),
                    ]
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
            positions: [ 2, 12 ],
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
