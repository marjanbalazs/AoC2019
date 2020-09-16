// Another Intcode Machine problem

// So this problem screams for a threadpool and a central thread that will string together the inputs and outputs between the stages.
// The threadpool will run intmachines and the central thread will submit jobs with a "phase" and "input" and expect an output.

use std::env;
use std::fs;

// Generate permutations
fn gen_permuations(inputs: Vec<i32>, res: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
    if inputs.len() == 0 {
        output.push(res.to_vec());
        return;
    }
    for elem in &inputs {
        let new_input = inputs.iter().filter(|x| *x != elem).cloned().collect();
        let mut new_res = res.clone();
        new_res.push(*elem);
        gen_permuations(new_input, &mut new_res, output);
    }
}

fn generate_permutation_iterative() {
    unimplemented!();
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

    let (file_path, input_val) = match vec_str.len() {
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

    let mem_original = parse_file_to_vec(file_path.to_string());

    let mut phase_permutations: Vec<Vec<i32>> = Vec::new();
    let phase_settings = Vec::from([0, 1, 2, 3, 4]);

    gen_permuations(phase_settings, &mut Vec::new(), &mut phase_permutations);

    let results: Vec<i32> = phase_permutations
        .into_iter()
        .map(|phase_settings| {
            let mut x = 0;
            for elem in phase_settings {
                let mut machine = intcode::Machine {
                    memory: &mut mem_original.clone(),
                    ip: 0,
                    input: Vec::from([x, elem]),
                    output: 0,
                };
                machine.run();
                x = machine.output;
            }
            x
        })
        .collect();

    println!("{:?}", results.iter().max().unwrap());

    Ok(())
}
