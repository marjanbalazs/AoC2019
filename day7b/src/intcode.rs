use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    Store,
    Load,
    JmpTrue,
    JmpFalse,
    Less,
    Equal,
    Halt,
}

#[derive(Debug)]
enum ArgMode {
    Position,
    Immediate,
}
#[derive(Debug)]
struct Command {
    op: Op,
    args: Option<Vec<ArgMode>>,
}

pub struct Machine<'a> {
    pub memory: &'a mut Vec<i32>,
    pub ip: usize,
    pub input: Receiver<i32>,
    pub output: Sender<i32>,
}

impl<'a> Machine<'a> {
    pub fn run(&mut self) {
        while self.ip < self.memory.len() {
            let op = decode(self.memory[self.ip]);
            let arg_len = match op.args {
                Some(x) => x.len(),
                None => 0,
            };
            let args = decode_argmodes(self.memory[self.ip], arg_len);
            match op.op {
                Op::Add => self.op_add(args),
                Op::Multiply => self.op_multiply(args),
                Op::Store => self.op_store(args),
                Op::Load => self.op_load(args),
                Op::JmpTrue => self.op_jmptrue(args),
                Op::JmpFalse => self.op_jmpfalse(args),
                Op::Less => self.op_less(args),
                Op::Equal => self.op_equal(args),
                Op::Halt => {
                    break;
                }
            }
        }
        drop(self);
    }

    fn op_add(&mut self, args: Vec<ArgMode>) {
        let lhs = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1],
        };

        let rhs = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2],
        };

        let res = lhs + rhs;

        match args[2] {
            ArgMode::Position => {
                let idx = self.memory[self.ip + 3 as usize];
                self.memory[idx as usize] = res;
            }
            ArgMode::Immediate => self.memory[self.ip + 3] = res,
        };
        self.ip += args.len() + 1;
    }

    fn op_multiply(&mut self, args: Vec<ArgMode>) {
        let lhs = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1],
        };

        let rhs = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2],
        };

        let res = lhs * rhs;

        match args[2] {
            ArgMode::Position => {
                let idx = self.memory[self.ip + 3 as usize];
                self.memory[idx as usize] = res;
            }
            ArgMode::Immediate => self.memory[self.ip + 3] = res,
        };
        self.ip += args.len() + 1;
    }

    fn op_store(&mut self, args: Vec<ArgMode>) {
        match args[0] {
            ArgMode::Position => {
                let index = self.memory[self.ip + 1] as usize;
                self.memory[index as usize] = self.input.recv().unwrap();
            }
            ArgMode::Immediate => {
                let index = self.ip + 1;
                self.memory[index as usize] = self.input.recv().unwrap();
            }
        }
        self.ip += args.len() + 1;
    }

    fn op_load(&mut self, args: Vec<ArgMode>) {
        let value = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1],
        };
        self.ip += args.len() + 1;
        self.output
            .send(value)
            .expect("I could not send a message on my output");
    }

    fn op_jmptrue(&mut self, args: Vec<ArgMode>) {
        let cond = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1 as usize],
        };

        let jmp = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2 as usize],
        };

        if cond != 0 {
            self.ip = jmp as usize;
        } else {
            self.ip += args.len() + 1;
        }
    }

    fn op_jmpfalse(&mut self, args: Vec<ArgMode>) {
        let cond = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1 as usize],
        };

        let jmp = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2 as usize],
        };

        if cond == 0 {
            self.ip = jmp as usize;
        } else {
            self.ip += args.len() + 1;
        }
    }

    fn op_less(&mut self, args: Vec<ArgMode>) {
        let lhs = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1 as usize],
        };

        let rhs = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2 as usize],
        };

        let store = if lhs < rhs { 1 } else { 0 };

        match args[2] {
            ArgMode::Position => {
                let index = self.memory[self.ip + 3] as usize;
                self.memory[index as usize] = store;
            }
            ArgMode::Immediate => {
                let index = self.ip + 3;
                self.memory[index as usize] = store;
            }
        }

        self.ip += args.len() + 1;
    }

    fn op_equal(&mut self, args: Vec<ArgMode>) {
        let lhs = match args[0] {
            ArgMode::Position => self.memory[self.memory[self.ip + 1 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 1 as usize],
        };

        let rhs = match args[1] {
            ArgMode::Position => self.memory[self.memory[self.ip + 2 as usize] as usize],
            ArgMode::Immediate => self.memory[self.ip + 2 as usize],
        };

        let store = if lhs == rhs { 1 } else { 0 };

        match args[2] {
            ArgMode::Position => {
                let index = self.memory[self.ip + 3] as usize;
                self.memory[index as usize] = store;
            }
            ArgMode::Immediate => {
                let index = self.ip + 3;
                self.memory[index as usize] = store;
            }
        }
        self.ip += args.len() + 1;
    }
}

fn decode_argmodes(opcode: i32, len: usize) -> Vec<ArgMode> {
    let mut argmodes: Vec<ArgMode> = Vec::new();
    let mut arg = opcode / 100;
    for _ in 0..len {
        match arg % 10 {
            1 => argmodes.push(ArgMode::Immediate),
            0 => argmodes.push(ArgMode::Position),
            _ => panic!("Unexpected stuff happened at argument mode deduction"),
        }
        arg /= 10;
    }
    argmodes
}

fn decode(opcode: i32) -> Command {
    let op: Command = match opcode % 100 {
        1 => {
            let arg_modes = decode_argmodes(opcode, 3);
            Command {
                op: Op::Add,
                args: Some(arg_modes),
            }
        }
        2 => {
            let arg_modes = decode_argmodes(opcode, 3);
            Command {
                op: Op::Multiply,
                args: Some(arg_modes),
            }
        }
        3 => {
            let arg_modes = decode_argmodes(opcode, 1);
            Command {
                op: Op::Store,
                args: Some(arg_modes),
            }
        }
        4 => {
            let arg_modes = decode_argmodes(opcode, 1);
            Command {
                op: Op::Load,
                args: Some(arg_modes),
            }
        }
        5 => {
            let arg_modes = decode_argmodes(opcode, 2);
            Command {
                op: Op::JmpTrue,
                args: Some(arg_modes),
            }
        }
        6 => {
            let arg_modes = decode_argmodes(opcode, 2);
            Command {
                op: Op::JmpFalse,
                args: Some(arg_modes),
            }
        }
        7 => {
            let arg_modes = decode_argmodes(opcode, 3);
            Command {
                op: Op::Less,
                args: Some(arg_modes),
            }
        }
        8 => {
            let arg_modes = decode_argmodes(opcode, 3);
            Command {
                op: Op::Equal,
                args: Some(arg_modes),
            }
        }
        99 => Command {
            op: Op::Halt,
            args: None,
        },
        x => {
            panic!("Unexpected instruction {:?}", x);
        }
    };
    op
}
