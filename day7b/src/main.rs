use std::env;
use std::fs;
use std::sync::mpsc::channel;
use std::thread;

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

    let mut phase_permutations: Vec<Vec<i32>> = Vec::new();
    let phase_settings = Vec::from([5, 6, 7, 8, 9]);

    let mem_original = parse_file_to_vec(file_path.to_string());
    gen_permuations(phase_settings, &mut Vec::new(), &mut phase_permutations);

    let results: Vec<i32> = phase_permutations
        .into_iter()
        .map(|phase_settings| {
            let mut x = 0;
            for elem in phase_settings {
                let (tx_main, rx_a) = channel();
                let (tx_a, rx_b) = channel();
                let (tx_b, rx_c) = channel();
                let (tx_c, rx_d) = channel();
                let (tx_d, rx_e) = channel();
                let (tx_e, rx_main) = channel();

                tx_main.send(0).unwrap();
                

                let mut mem_a = mem_original.to_vec();
                let thread_a = thread::spawn(move || {
                    let mut machine = intcode::Machine {
                        memory: &mut mem_a,
                        ip: 0,
                        input: Vec::from([x, elem]),
                        output: 0,
                    };
                    machine.run();
                    x = machine.output;
                });

                let mut mem_b = mem_original.to_vec();
                let thread_b = thread::spawn(move || {
                    let mut machine = intcode::Machine {
                        memory: &mut mem_b,
                        ip: 0,
                        input: Vec::from([x, elem]),
                        output: 0,
                    };
                    machine.run();
                    x = machine.output;
                });

                let mut mem_c = mem_original.to_vec();
                let thread_c = thread::spawn(move || {
                    let mut machine = intcode::Machine {
                        memory: &mut mem_c,
                        ip: 0,
                        input: Vec::from([x, elem]),
                        output: 0,
                    };
                    machine.run();
                    x = machine.output;
                });

                let mut mem_d = mem_original.to_vec();
                let thread_d = thread::spawn(move || {
                    let mut machine = intcode::Machine {
                        memory: &mut mem_d,
                        ip: 0,
                        input: Vec::from([x, elem]),
                        output: 0,
                    };
                    machine.run();
                    x = machine.output;
                });

                let mut mem_e = mem_original.to_vec();
                let thread_e = thread::spawn(move || {
                    let mut machine = intcode::Machine {
                        memory: &mut mem_e,
                        ip: 0,
                        input: Vec::from([x, elem]),
                        output: 0,
                    };
                    machine.run();
                    x = machine.output;
                });
                let x = rx_main.recv().unwrap();
                println!("Received in main: {:?}", x);

                thread_a.join().unwrap();
                thread_b.join().unwrap();
                thread_c.join().unwrap();
                thread_d.join().unwrap();
                thread_e.join().unwrap();
            }
            x
        })
        .collect();

    println!("{:?}", results.iter().max().unwrap());

    /*
    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        tx.send("Hello, thread".to_owned())
            .expect("Unable to send on channel");
    });

    let receiver = thread::spawn(move || {
        let value = rx.recv().expect("Unable to receive from channel");
        println!("{}", value);
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");

    */

    Ok(())
}
