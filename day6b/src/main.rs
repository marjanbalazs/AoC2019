use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn add_connection(connection: String, map: &mut HashMap<String, String>) {
    let x: Vec<&str> = connection.split(')').collect();
    let parent = x[0].to_string();
    let child = x[1].to_string();
    map.insert(child, parent);
}

fn find_path_len(a: &str, b: &str, map: &HashMap<String, String>) -> usize {
    let mut a_path = Vec::new();
    let mut b_path = Vec::new();

    let mut search = a;

    loop {
        search = map.get(search).unwrap();
        a_path.push(search);
        if search.eq("COM") {
            break;
        }
    }

    search = b;

    loop {
        search = map.get(search).unwrap();
        b_path.push(search);
        if search.eq("COM") || a_path.contains(&search) {
            break;
        }
    }

    let idx = a_path.iter().position(|a| a == b_path.last().unwrap());
    let _a_first_split = a_path.split_off(idx.unwrap());
    b_path.append(&mut a_path);

    b_path.len() - 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
    let mut map: HashMap<String, String> = HashMap::new();

    for line in file.lines() {
        match line {
            Ok(s) => {
                add_connection(s, &mut map);
            }
            Err(e) => print!("{}", e),
        }
    }

    let sol = find_path_len(&"SAN".to_string(), &"YOU".to_string(), &map);

    println!("{}", sol);

}

