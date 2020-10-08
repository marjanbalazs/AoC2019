use std::sync::mpsc::channel;
use std::thread;
use std::env;
use std::fs;

fn parse_file_to_vec(file_path: String) -> Vec<i32> {
    let content = fs::read_to_string(file_path).unwrap();
    let tokens: Vec<&str> = content.split(',').collect();

    let v: Vec<i32> = tokens
        .into_iter()
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect();

    v
}

fn main() {

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

    let mut mem_original = parse_file_to_vec(file_path.to_string());

    let (tx_from_main, rx_from_main) = channel();
    let (tx_from_machine, rx_from_machine) = channel();
    thread::spawn(move || {
        let mut machine = intcode::Machine{
            memory: &mut mem_original,
            ip: 0,
            input: rx_from_main,
            output: tx_from_machine,
        };
    });
}
