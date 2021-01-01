use std::io::BufRead;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Inst {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}

type Program = Vec<Inst>;

#[derive(Default, Clone, Debug)]
pub struct InstMeta {
    pub execution_count: usize,
}

pub struct Console {
    pub cycles: i32,
    pub accumulator: i32,
    program: Vec<Inst>,
    trace: Vec<InstMeta>,
    pc: usize,
}

impl Console {
    pub fn new() -> Console {
        Console {
            cycles: 0,
            accumulator: 0,
            program: vec![],
            trace: vec![],
            pc: 0,
        }
    }

    pub fn load_program(&mut self, code: Program) {
        self.program = code.clone();
        self.trace = vec![InstMeta::default(); code.len()];
        self.pc = 0;
        self.accumulator = 0;
    }

    // Returns true is program completes successfully, false if a break
    // condition was met.
    pub fn run_until(&mut self, breakpoint: fn(&Inst, &InstMeta) -> bool) -> bool {
        loop {
            if self.pc == self.program.len() {
                return true;
            } else if breakpoint(self.inst(), self.meta()) {
                return false;
            } else {
                self.step();
            }
        }
    }

    fn inst(&self) -> &Inst {
        self.program.get(self.pc).unwrap()
    }

    fn meta(&self) -> &InstMeta {
        self.trace.get(self.pc).unwrap()
    }

    fn step(&mut self) {
        self.trace.get_mut(self.pc).unwrap().execution_count += 1;
        self.cycles += 1;

        match self.inst() {
            Inst::Nop(_) => self.pc += 1,
            Inst::Acc(value) => {
                self.accumulator += *value as i32;
                self.pc += 1;
            }
            Inst::Jmp(value) => {
                if *value > 0 {
                    self.pc = self.pc.wrapping_add(*value as usize);
                } else {
                    self.pc = self.pc.wrapping_sub(value.abs() as usize);
                }
            }
        }
    }
}

pub fn parse_program<T: BufRead>(input: T) -> Program {
    input.lines().map(|line| {
        let line = line.unwrap();
        let mut fields = line.split(" ");

        let inst = fields.next().unwrap();
        let arg = fields.next().unwrap();

        match inst {
            "nop" => Inst::Nop(arg.parse::<i16>().unwrap()),
            "acc" => Inst::Acc(arg.parse::<i16>().unwrap()),
            "jmp" => Inst::Jmp(arg.parse::<i16>().unwrap()),
            _ => panic!(format!("Unexpected instruction: {} {}", inst, arg))
        }
    }).collect()
}

#[test]
fn test_loop_detection() {
    let mut console = Console::new();
    let program = parse_program(std::io::Cursor::new("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"));
    console.load_program(program);

    let successful = console.run_until(|_inst, meta| meta.execution_count > 0);

    assert_eq!(5, console.accumulator);
    assert_eq!(false, successful);
}

#[test]
fn test_successful_execution() {
    let mut console = Console::new();
    let program = parse_program(std::io::Cursor::new("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6"));
    console.load_program(program);

    let successful = console.run_until(|_inst, meta| meta.execution_count > 0);

    assert_eq!(8, console.accumulator);
    assert_eq!(true, successful);
}

