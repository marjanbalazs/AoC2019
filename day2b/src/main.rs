/* Seems to me this is a machine that manipulates numbers on a tape,
going back and forward....
*/

/* Plan:
1, Read the whole file into a vector
2, Step around in the vector reading opcodes
*/
use std::env;
use std::fs;

const TARGET: u32 = 19690720;

fn op_add(pos: usize, vec: &mut Vec<u32>) {
    let lhs = vec[pos + 1];
    let rhs = vec[pos + 2];
    let res = vec[pos + 3];

    vec[res as usize] = vec[lhs as usize] + vec[rhs as usize];
}

fn op_multiply(pos: usize, vec: &mut Vec<u32>) {
    let lhs = vec[pos + 1];
    let rhs = vec[pos + 2];
    let res = vec[pos + 3];

    vec[res as usize] = vec[lhs as usize] * vec[rhs as usize];
}

fn process_ops(vec: &mut Vec<u32>) {
    let mut x: usize = 0;
    while x < vec.len() {
        match vec[x] {
            1 => op_add(x, vec),
            2 => op_multiply(x, vec),
            99 => break,
            _ => break,
        }
        x += 4;
    }
}

fn main() -> Result<(), ()> {
    let vec_str: Vec<String> = env::args().collect();

    let file_path = match vec_str.len() {
        0..=1 => Err(()),
        2 => Ok(vec_str.get(1).unwrap()),
        _ => Err(()),
    };

    let content = fs::read_to_string(file_path.unwrap()).unwrap();

    let tokens: Vec<&str> = content.split(",").collect();
    let mut vec_num: Vec<u32> = Vec::new();

    for elem in tokens {
        vec_num.push(elem.trim().parse::<u32>().unwrap());
    }

    let mut vec = vec_num.clone();

    /* New code here */
    for i in 0..100 {
        for j in 0..100 {
            // reset memory here
            vec[1] = i;
            vec[2] = j;
            process_ops(&mut vec);
            if vec[0] == TARGET {
                println!("{}", 100*i+j);
            }
            vec = vec_num.clone();
        }
    }
    Ok(())
}
