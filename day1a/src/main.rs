use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn fuel_requirement(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let file = File::open("C:/Users/pc/Desktop/Dev/AoC_2019/day1a/src/puzzle_input.txt");
    let file = BufReader::new(file.unwrap());

    let res: u32 = file
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<u32>().unwrap())
        .map(|s| (s / 3) - 2)
        .sum();

    // Parsing the integer could fail
    // Substraction could fail

    println!("{}", res);

    Ok(())
}
