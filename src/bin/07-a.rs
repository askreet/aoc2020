use aoc2020::aoc_07::BagGraph;
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let bags = BagGraph::from(reader).could_hold("shiny gold");

    for name in &bags {
        println!(" - {}", name);
    }

    println!("Total: {}", bags.len());
}
