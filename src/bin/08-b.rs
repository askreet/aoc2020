use aoc2020::aoc_08::{Console, Inst, parse_program};
use aoc2020::create_input_reader;

fn main() {
    let reader = create_input_reader();

    let mut console = Console::new();
    let program = parse_program(reader);

    for (idx, inst) in program.iter().enumerate() {
        let swap_inst = match inst {
            Inst::Nop(value) => Inst::Jmp(*value),
            Inst::Acc(_) => continue,
            Inst::Jmp(value) => Inst::Nop(*value)
        };

        println!("Attempting exection with instruction #{} swapped from {:?} to {:?}.", idx, inst, swap_inst);

        let mut new_program = program.clone();
        new_program[idx] = swap_inst;

        console.load_program(new_program);
        let success = console.run_until(|_inst, meta| meta.execution_count > 0);
        if success {
            println!("Successfully exited program with acc={}.", console.accumulator);
        }
    }
}
