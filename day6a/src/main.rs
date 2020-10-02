use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn node_add(connection: String, map: &mut HashMap<String, Vec<String>>) {
    let x: Vec<&str> = connection.split(')').collect();
    if map.contains_key(&x[0].to_string()) {
        map.entry(x[0].to_string())
            .and_modify(|n| n.push(x[1].to_string()));
    } else {
        let mut vec: Vec<String> = Vec::new();
        vec.push(x[1].to_string());
        map.insert(x[0].to_string(), vec);
    }
}

fn calc_all_connections(
    map: &HashMap<String, Vec<String>>,
    node: &String,
    start_weight: usize,
) -> usize {
    let mut acc = start_weight;
    println!("{}", acc);

    match map.get(node) {
        Some(value) => {
            if value.is_empty() {
                return acc;
            } else {
                for elem in map.get(node).unwrap() {
                    acc += calc_all_connections(map, elem, start_weight + 1);
                }
            }
        }
        None => return acc,
    }
    acc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in file.lines() {
        match line {
            Ok(s) => {
                node_add(s, &mut map);
            }
            Err(e) => print!("{}", e),
        }
    }

    for key in map.keys() {
        let f = map.get_key_value(key).unwrap();
        println!("Key: {} Children:: {:?}", f.0, f.1);
    }

    println!("{}", calc_all_connections(&map, &"COM".to_string(), 0));
}
