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
    Add {
        lhs: OpMode,
        rhs: OpMode,
        out: OpMode,
    },
    Multiply {
        lhs: OpMode,
        rhs: OpMode,
        out: OpMode,
    },
    Store {
        input: OpMode,
    },
    Load {
        input: OpMode,
    },
}

enum OpMode {
    Position,
    Immediate,
}

struct Command {
    op: Op,
    args: Vec<i32>,
}

fn op_add(pos: usize, vec: &mut Vec<i32>) {
    let lhs = vec[pos + 1];
    let rhs = vec[pos + 2];
    let res = vec[pos + 3];

    vec[res as usize] = vec[lhs as usize] + vec[rhs as usize];
}

fn op_multiply(pos: usize, vec: &mut Vec<i32>) {
    let lhs = vec[pos + 1];
    let rhs = vec[pos + 2];
    let res = vec[pos + 3];

    vec[res as usize] = vec[lhs as usize] * vec[rhs as usize];
}

fn op_store(pos: usize, input: i32, vec: &mut Vec<i32>) {
    let index = vec[pos + 1];
    vec[index as usize] = input;
}

fn op_load(pos: usize, vec: &mut Vec<i32>) -> i32 {
    let index = vec[pos + 1];
    return vec[index as usize];
}

fn process(input: i32, vec: &mut Vec<i32>) -> i32 {
    let mut x: usize = 0;
    let mut output = 0;
    while x < vec.len() {
        match vec[x] {
            1 => op_add(x, vec),
            2 => op_multiply(x, vec),
            3 => op_store(x, input, vec),
            4 => output = op_load(x, vec),
            99 => break,
            _ => break,
        }
        // Instruction pointer adjustment
        x += 4;
    }
    output
}


fn execute_opcode(op: Command) {

}


fn decode_opcode(opcode: i32) -> Command {
    let mut cmd: Command = match opcode % 100 {
        0 => {
            unimplemented!();
        }
        1 => {
            unimplemented!();
        }
        2 => {
            unimplemented!();
        }
        99 => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    };
    return cmd;
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
            panic!("No args");
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
