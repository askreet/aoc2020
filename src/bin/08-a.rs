use aoc2020::aoc_08::{Console, parse_program};
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let mut console = Console::new();
    let program = parse_program(reader);
    console.load_program(program);
    console.run_until(|_inst, meta| meta.execution_count > 0);

    println!("Executed {} instructions, acc={}", console.cycles, console.accumulator);
}
