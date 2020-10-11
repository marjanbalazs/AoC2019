use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc::channel;
use std::thread;

fn parse_file_to_vec(file_path: String) -> HashMap<usize, i64> {
    let content = fs::read_to_string(file_path).unwrap();
    let tokens: Vec<&str> = content.split(',').collect();
    let mut ret: HashMap<usize, i64> = HashMap::new();
    for elem in tokens.iter().enumerate() {
        ret.insert(elem.0, elem.1.trim().parse::<i64>().unwrap());
    }
    ret
}

fn main() {
    let vec_str: Vec<String> = env::args().collect();

    let (file_path, input_val) = match vec_str.len() {
        0..=2 => {
            panic!("Not enough args");
        }
        3 => (
            vec_str.get(1).unwrap(),
            (*vec_str.get(2).unwrap()).trim().parse::<i64>().unwrap(),
        ),
        _ => {
            panic!("Too many args");
        }
    };

    let mut mem_original = parse_file_to_vec(file_path.to_string());

    let (tx_from_main, rx_from_main) = channel();
    let (tx_from_machine, rx_from_machine) = channel();
    tx_from_main.send(input_val).expect("Error at sending");

    thread::spawn(move || {
        let mut machine = intcode::Machine::new(&mut mem_original, rx_from_main, tx_from_machine);
        machine.run();
    });

    let val = rx_from_machine.recv().expect("Error from machine");
    println!("Value: {:?}", val);
}
