/* Seems to me this is a machine that manipulates numbers on a tape,
going back and forward....
*/

/* Plan:
1, Read the whole file into a vector
2, Step around in the vector reading opcodes
*/

/*
First, you'll need to add two new instructions:

    Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
    Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

Programs that use these instructions will come with documentation that explains what should be connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input, then halts.

Second, you'll need to add support for parameter modes:

Each parameter of an instruction is handled based on its parameter mode. Right now, your ship computer already understands parameter mode 0, position mode, which causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory. Until now, all parameters have been in position mode.

Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
*/

use std::env;
use std::fs;

enum Op {
    Add,
    Multiply,
    Store,
    Load,
    Halt,
}

#[derive(Debug)]
enum ArgMode {
    Position,
    Immediate,
}

struct Command {
    op: Op,
    args: Option<Vec<ArgMode>>,
}

fn op_add(pos: usize, args: Vec<ArgMode>, vec: &mut Vec<i32>) {
    let lhs = match args[0] {
        ArgMode::Position => vec[vec[pos + 1 as usize] as usize],
        ArgMode::Immediate => vec[pos + 1],
    };

    let rhs = match args[1] {
        ArgMode::Position => vec[vec[pos + 2 as usize] as usize],
        ArgMode::Immediate => vec[pos + 2],
    };

    let res = lhs + rhs;

    match args[2] {
        ArgMode::Position => {
            let idx = vec[pos + 3 as usize];
            vec[idx as usize] = res;
        }
        ArgMode::Immediate => vec[pos + 3] = res,
    };
}

fn op_multiply(pos: usize, args: Vec<ArgMode>, vec: &mut Vec<i32>) {
    let lhs = match args[0] {
        ArgMode::Position => vec[vec[pos + 1 as usize] as usize],
        ArgMode::Immediate => vec[pos + 1],
    };

    let rhs = match args[1] {
        ArgMode::Position => vec[vec[pos + 2 as usize] as usize],
        ArgMode::Immediate => vec[pos + 2],
    };

    let res = lhs * rhs;

    match args[2] {
        ArgMode::Position => {
            let idx = vec[pos + 3 as usize];
            vec[idx as usize] = res;
        }
        ArgMode::Immediate => vec[pos + 3] = res,
    };
}

fn op_store(pos: usize, input: i32, args: Vec<ArgMode>, vec: &mut Vec<i32>) {
    match args[0] {
        ArgMode::Position => {
            let index = vec[pos + 1] as usize;
            vec[index as usize] = input;
        }
        ArgMode::Immediate => {
            let index = pos + 1;
            vec[index as usize] = input;
        }
    }
}

fn op_load(pos: usize, args: Vec<ArgMode>, vec: &mut Vec<i32>) -> i32 {
    match args[0] {
        ArgMode::Position => vec[vec[pos + 1] as usize],
        ArgMode::Immediate => vec[pos + 1],
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

fn process(input: i32, ops: &mut Vec<i32>) -> i32 {
    let mut output = 0;
    let mut i = 0;
    while i < ops.len() {
        let op = decode(ops[i]);
        let arg_len = match op.args {
            Some(x) => x.len(),
            None => 0,
        };
        let argmodes = decode_argmodes(ops[i], arg_len);
        match op.op {
            Op::Add => {
                op_add(i, argmodes, ops);
            }
            Op::Multiply => {
                op_multiply(i, argmodes, ops);
            }
            Op::Store => {
                op_store(i, input, argmodes, ops);
            }
            Op::Load => output = op_load(i, argmodes, ops),
            Op::Halt => {
                break;
            }
        }

        i += arg_len + 1;
    }
    output
}

fn parse_file_to_vec(file_path: String) -> Vec<i32> {
    let content = fs::read_to_string(file_path).unwrap();
    let tokens: Vec<&str> = content.split(',').collect();

    let v: Vec<i32> = tokens
        .into_iter()
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect();

    v
}

fn main() -> Result<(), ()> {
    let vec_str: Vec<String> = env::args().collect();

    let (file_path, input) = match vec_str.len() {
        0..=2 => {
            panic!("Not enough args");
        }
        3 => (
            vec_str.get(1).unwrap(),
            (*vec_str.get(2).unwrap()).trim().parse::<i32>().unwrap(),
        ),
        _ => {
            panic!("Too many args");
        }
    };

    let mut vec = parse_file_to_vec(file_path.to_string());

    let output = process(input, &mut vec);

    println!("{}", output);

    Ok(())
}
