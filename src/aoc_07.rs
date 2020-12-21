use std::collections::HashMap;
use std::io::BufRead;
use regex::{Regex, Captures};
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BagRule {
    desc: String,
    can_hold: HashMap<String, u8>,
}

pub fn parse_rules<T: BufRead>(input: T) -> Vec<BagRule> {
    let mut result = Vec::new();

    input.lines().for_each(|line| {});

    result
}

fn parse_rule(input: String) -> BagRule {
    let bag = regex_captures!(r"^(\w+ \w+) bags", &input).unwrap()[1].to_string();


    let bag_re: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    BagRule {
        desc: bag,
        can_hold: bag_re.captures_iter(&input)
            .map(|c| (c[2].to_string(), c[1].parse::<u8>().unwrap()) )
            .collect(),
    }
}

#[test]
fn test_parse_rule() {
    assert_eq!(
        parse_rule("shiny aqua bags contain 1 dark white bag.\n".to_string()),
        BagRule {
            desc: "shiny aqua".to_string(),
            can_hold: hashmap! { "dark white".to_string() => 1 },
        }
    );

    assert_eq!(
        parse_rule("dark purple bags contain 1 wavy indigo bag, 3 bright black bags, 3 dotted teal bags.".to_string()),
        BagRule {
            desc: "dark purple".to_string(),
            can_hold: hashmap! {
                "wavy indigo".to_string() => 1,
                "bright black".to_string() => 3,
                "dotted teal".to_string() => 3,
            },
        }
    );

    assert_eq!(
        parse_rule("dull silver bags contain no other bags.".to_string()),
        BagRule {
            desc: "dull silver".to_string(),
            can_hold: hashmap! {},
        }
    );
}

