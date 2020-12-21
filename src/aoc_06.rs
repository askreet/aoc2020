use itertools::{Itertools, assert_equal};
use std::collections::{HashMap, HashSet};

pub fn unique_chars(input: &str) -> Vec<char> {
    input.chars().unique().filter(|c| *c != '\n').collect()
}

pub fn all_answered_yes(input: &str) -> HashSet<char> {
    let mut line_reader = input.lines();

    let init: HashSet<char> = line_reader.next().expect("Must have at least one line!").chars().collect();

    line_reader
        .filter(|line| *line != "")
        .fold(init, |mut a, e| a.intersection(&e.chars().collect()).copied().collect())
}

#[test]
fn test_unique_chars() {
    assert_eq!(vec!['a', 'b', 'c', 'x', 'y', 'z'], unique_chars("abcx\nabcy\nabcz\n"));
}

#[test]
fn test_all_answered_yes() {
    assert_eq!(hashset!['a', 'b', 'c'], all_answered_yes("abc\n"));
    assert_eq!(hashset![], all_answered_yes("a\nb\nc\n"));
    assert_eq!(hashset!['a'], all_answered_yes("ab\nac\n"));
    assert_eq!(hashset!['a'], all_answered_yes("a\na\na\na\n"));
    assert_eq!(hashset!['m', 'r', 'c', 'q', 'l', 'o'], all_answered_yes("rmcqdblnto\nqlcnmor\nrhmzalcsoq\nlgxcrmnqovd\n\n"));
}