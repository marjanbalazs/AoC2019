use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::convert::*;


fn fuel_requirement_iter(mass: i64) -> i64 {
    let mut ret = 0;
    let mut mass_calc: i64 = mass;
    loop {
        mass_calc = ( mass_calc / 3 ) - 2 ;
        ret += mass_calc;
        if mass_calc < 0 {
            break;
        }
    }
    return ret;
}

fn fuel_requirement_recursive(mass: i64) -> i64 {
    let fuel = (mass / 3) -2 ;
    if fuel < 0 {
        return 0;
    }
    return fuel + fuel_requirement_recursive(fuel);
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let file = File::open("C:/Users/pc/Desktop/Dev/AoC_2019/day1b/src/puzzle_input.txt");
    let file = BufReader::new(file.unwrap());

    let res_recursive: i64 = file
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<i64>().unwrap())
        .map(|s| fuel_requirement_recursive(s))
        .sum();


    println!("Recursive: {}", res_recursive);

    Ok(())
}
