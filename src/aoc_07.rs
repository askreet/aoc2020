use std::collections::HashMap;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

type BagIndex = usize;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BagRule {
    desc: String,
    can_hold: HashMap<String, u8>,
}

#[derive(Debug)]
struct BagGraphNode {}

#[derive(Debug)]
struct BagGraphEdge {
    idx: BagIndex,
    count: u8,
}

#[derive(Debug, Default)]
pub struct BagGraph {
    // Allow name lookup for linking held bags.
    names: HashMap<String, BagIndex>,
    bags: Vec<BagGraphNode>,
    links: HashMap<BagIndex, Vec<BagGraphEdge>>,
}

impl BagGraph {
    fn link(&mut self, bag_desc: &String, holds: &String, count: u8) {
        let source_bag_id = self.find_or_create_idx(bag_desc);
        let held_bag_id = self.find_or_create_idx(holds);

        let new_edge = BagGraphEdge { idx: held_bag_id, count: count };

        if !self.links.contains_key(&source_bag_id) {
            self.links.insert(source_bag_id, vec![new_edge]);
        } else {
            self.links.get_mut(&source_bag_id).unwrap().push(new_edge);
        }
    }

    pub fn can_eventually_hold(&mut self, bag_desc: &str, target: &str) -> bool {
        let needle = self.find_or_create_idx(&target.to_string());
        let haystack = self.find_or_create_idx(&bag_desc.to_string());

        self.can_eventually_hold_by_idx(needle, haystack)
    }

    fn can_eventually_hold_by_idx(&self, needle: usize, haystack: usize) -> bool {
        let mut searched: Vec<usize> = vec![];
        let mut frontier = vec![haystack];

        while let Some(bag_id) = frontier.pop() {
            searched.push(bag_id);

            if let Some(targets) = self.links.get(&bag_id) {
                for target in targets {
                    if target.idx == needle {
                        return true;
                    }

                    if !frontier.contains(&target.idx) {
                        frontier.push(target.idx);
                    }
                }
            }
        }

        return false;
    }

    pub fn could_hold(&self, bag_desc: &str) -> Vec<String> {
        let mut result = vec![];
        let needle = self.names.get(bag_desc).unwrap();

        for (name, idx) in &self.names {
            if self.can_eventually_hold_by_idx(*needle, *idx) {
                result.push(name.clone());
            }
        }

        result
    }

    pub fn inner_bag_count(&self, bag_desc: &str) -> usize {
        let idx = self.names.get(bag_desc).unwrap();

        self.inner_bag_count_idx(*idx)
    }

    fn inner_bag_count_idx(&self, idx: BagIndex) -> usize {
        match self.links.get(&idx) {
            None => 0, // The bag cannot hold anything.
            Some(target_bags) =>
                target_bags
                    .iter()
                    .map(|t| ((self.inner_bag_count_idx(t.idx) + 1) * t.count as usize))
                    .sum()
        }
    }

    fn find_or_create_idx(&mut self, bag_desc: &String) -> BagIndex {
        if !self.names.contains_key(bag_desc) {
            let val = self.bags.len();

            self.bags.push(BagGraphNode {});

            self.names.insert(bag_desc.clone(), val);
        }

        self.names.get(bag_desc).unwrap().clone()
    }

    pub fn from<T: BufRead>(input: T) -> BagGraph {
        let mut result = BagGraph::default();

        input.lines().for_each(|line| {
            let rule = parse_rule(line.unwrap());

            for (held_bag, count) in &rule.can_hold {
                result.link(&rule.desc, held_bag, *count);
            }
        });

        result
    }
}

#[test]
fn test_bag_graph() {
    let input = std::io::Cursor::new("\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.");

    let mut ruleset = BagGraph::from(input);

    assert_eq!(true, ruleset.can_eventually_hold("light red", "shiny gold"));
    assert_eq!(true, ruleset.can_eventually_hold("dark orange", "shiny gold"));
    assert_eq!(true, ruleset.can_eventually_hold("bright white", "shiny gold"));
    assert_eq!(true, ruleset.can_eventually_hold("muted yellow", "shiny gold"));
    assert_eq!(false, ruleset.can_eventually_hold("shiny gold", "shiny gold"));
    assert_eq!(false, ruleset.can_eventually_hold("dark olive", "shiny gold"));
    assert_eq!(false, ruleset.can_eventually_hold("vibrant plum", "shiny gold"));
    assert_eq!(false, ruleset.can_eventually_hold("faded blue", "shiny gold"));
    assert_eq!(false, ruleset.can_eventually_hold("dotted black", "shiny gold"));

    let mut list = ruleset.could_hold("shiny gold");
    list.sort();

    assert_eq!(
        vec!["bright white", "dark orange", "light red", "muted yellow"],
        list
    );

    assert_eq!(32, ruleset.inner_bag_count("shiny gold"));
}

fn parse_rule(input: String) -> BagRule {
    let bag = regex_captures!(r"^(\w+ \w+) bags", &input).unwrap()[1].to_string();

    let bag_re: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    BagRule {
        desc: bag,
        can_hold: bag_re.captures_iter(&input)
            .map(|c| (c[2].to_string(), c[1].parse::<u8>().unwrap()))
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

