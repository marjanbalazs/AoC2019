use std::collections::HashMap;
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
    SetBase,
}

#[derive(Debug)]
enum ArgMode {
    Position,
    Immediate,
    Relative,
}
#[derive(Debug)]
struct Command {
    op: Op,
    args: Option<Vec<ArgMode>>,
}

pub struct Machine {
    pub input: Receiver<i64>,
    pub output: Sender<i64>,
    memory: HashMap<usize, i64>,
    base: i64,
    ip: usize,
}

impl Machine {
    pub fn run(&mut self) {
        self.base = 0;
        while self.ip < self.memory.len() {
            let op = decode(*self.memory.get(&self.ip).unwrap_or(&0));
            match op.op {
                Op::Add => self.op_add(op.args.unwrap()),
                Op::Multiply => self.op_multiply(op.args.unwrap()),
                Op::Store => self.op_store(op.args.unwrap()),
                Op::Load => self.op_load(op.args.unwrap()),
                Op::JmpTrue => self.op_jmptrue(op.args.unwrap()),
                Op::JmpFalse => self.op_jmpfalse(op.args.unwrap()),
                Op::Less => self.op_less(op.args.unwrap()),
                Op::Equal => self.op_equal(op.args.unwrap()),
                Op::SetBase => self.op_setbase(op.args.unwrap()),
                Op::Halt => {
                    break;
                }
            }
        }
    }

    pub fn new(
        mem: &mut HashMap<usize, i64>,
        input: Receiver<i64>,
        output: Sender<i64>,
    ) -> Machine {
        Machine {
            base: 0,
            ip: 0,
            input,
            output,
            memory: mem.clone(),
        }
    }

    fn get_val(&self, pos: usize, mode: &ArgMode) -> i64 {
        let val = match *mode {
            ArgMode::Position => {
                let idx = *self.memory.get(&pos).unwrap_or(&0);
                *self.memory.get(&(idx as usize)).unwrap_or(&0)
            }
            ArgMode::Immediate => *self.memory.get(&pos).unwrap_or(&0),
            ArgMode::Relative => {
                let idx = *self
                    .memory
                    .get(&pos)
                    .unwrap_or(&0);
                *self.memory.get(&((idx as i64 + self.base) as usize)).unwrap_or(&0)
            }
        };
        val
    }

    fn set_val(&mut self, val: i64, pos: usize, mode: &ArgMode) {
        match *mode {
            ArgMode::Position => {
                let idx = *self.memory.get(&pos).unwrap_or(&0);
                self.memory.insert(idx as usize, val);
            }
            ArgMode::Immediate => {
                self.memory.insert(pos, val);
            }
            ArgMode::Relative => {
                let idx = *self
                    .memory
                    .get(&pos)
                    .unwrap_or(&0);
                self.memory.insert((idx + self.base) as usize, val);
            }
        }
    }

    fn op_add(&mut self, args: Vec<ArgMode>) {
        let lhs = self.get_val(self.ip + 1, &args[0]);
        let rhs = self.get_val(self.ip + 2, &args[1]);
        let res = lhs + rhs;
        self.set_val(res, self.ip + 3, &args[2]);
        self.ip += args.len() + 1;
    }

    fn op_multiply(&mut self, args: Vec<ArgMode>) {
        let lhs = self.get_val(self.ip + 1, &args[0]);
        let rhs = self.get_val(self.ip + 2, &args[1]);
        let res = lhs * rhs;
        self.set_val(res, self.ip + 3, &args[2]);
        self.ip += args.len() + 1;
    }

    fn op_store(&mut self, args: Vec<ArgMode>) {
        let res = self.input.recv().unwrap();
        self.set_val(res, self.ip + 1, &args[0]);
        self.ip += args.len() + 1;
    }

    fn op_load(&mut self, args: Vec<ArgMode>) {
        let value = self.get_val(self.ip + 1, &args[0]);
        self.ip += args.len() + 1;
        self.output
            .send(value)
            .expect("I could not send a message on my output");
    }

    fn op_jmptrue(&mut self, args: Vec<ArgMode>) {
        let cond = self.get_val(self.ip + 1, &args[0]);
        let jmp = self.get_val(self.ip + 2, &args[1]);
        if cond != 0 {
            self.ip = jmp as usize;
        } else {
            self.ip += args.len() + 1;
        }
    }

    fn op_jmpfalse(&mut self, args: Vec<ArgMode>) {
        let cond = self.get_val(self.ip + 1, &args[0]);
        let jmp = self.get_val(self.ip + 2, &args[1]);
        if cond == 0 {
            self.ip = jmp as usize;
        } else {
            self.ip += args.len() + 1;
        }
    }

    fn op_less(&mut self, args: Vec<ArgMode>) {
        let lhs = self.get_val(self.ip + 1, &args[0]);
        let rhs = self.get_val(self.ip + 2, &args[1]);
        let store = if lhs < rhs { 1 } else { 0 };
        self.set_val(store, self.ip + 3, &args[2]);
        self.ip += args.len() + 1;
    }

    fn op_equal(&mut self, args: Vec<ArgMode>) {
        let lhs = self.get_val(self.ip + 1, &args[0]);
        let rhs = self.get_val(self.ip + 2, &args[1]);
        let store = if lhs == rhs { 1 } else { 0 };
        self.set_val(store, self.ip + 3, &args[2]);
        self.ip += args.len() + 1;
    }

    fn op_setbase(&mut self, args: Vec<ArgMode>) {
        let val = self.get_val(self.ip + 1, &args[0]);
        self.base += val;
        self.ip += args.len() + 1;
    }
}

fn decode_argmodes(opcode: i64, len: usize) -> Vec<ArgMode> {
    let mut argmodes: Vec<ArgMode> = Vec::new();
    let mut arg = opcode / 100;
    for _ in 0..len {
        match arg % 10 {
            2 => argmodes.push(ArgMode::Relative),
            1 => argmodes.push(ArgMode::Immediate),
            0 => argmodes.push(ArgMode::Position),
            _ => panic!("Unexpected stuff happened at argument mode deduction"),
        }
        arg /= 10;
    }
    argmodes
}

fn decode(opcode: i64) -> Command {
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
        9 => {
            let arg_modes = decode_argmodes(opcode, 1);
            Command {
                op: Op::SetBase,
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
