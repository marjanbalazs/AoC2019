/* Seems to me this is a machine that manipulates numbers on a tape,
going back and forward....
*/

/* Plan:
1, Read the whole file into a vector
2, Step around in the vector reading opcodes
*/
use std::env;
use std::fs;

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

fn process_ops(input: i32, vec: &mut Vec<i32>) -> i32 {
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

fn parse_file_to_vec(file_path: String) -> Vec<i32> {
    let content = fs::read_to_string(file_path).unwrap();
    let tokens: Vec<&str> = content.split(",").collect();

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

    let output = process_ops(input, &mut vec);

    println!("{}", output);

    Ok(())
}
