use aoc2020::aoc_07::BagGraph;
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let bags = BagGraph::from(reader);
    println!("Shiny bag must contain: {}", bags.inner_bag_count("shiny gold"));
}
