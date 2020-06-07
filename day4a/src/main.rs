const INPUT: &'static str = "136760-595730";

/*
It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
*/

fn adjecent_digits(num: &Vec<u32>) -> bool {
    let mut ret: bool = false;
    for x in 0..(num.len() - 1) {
        if num[x] == num[x + 1] {
            ret = true;
            break;
        }
    }
    ret
}

fn non_decreasing(num: &Vec<u32>) -> bool {
    let mut ret: bool = true;
    for x in 0..(num.len() - 1) {
        if num[x + 1] < num[x] {
            ret = false;
            break;
        }
    }
    ret
}

fn generate_digits(num: u32) -> Vec<u32> {
    let x: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    x
}

fn main() {
    let limits = INPUT
        .split("-")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    let mut pass_cardinality: u32 = 0;

    for x in *limits.get(0).unwrap()..*limits.get(1).unwrap() {
        let candidate = generate_digits(x);
        if non_decreasing(&candidate) && adjecent_digits(&candidate) {
            pass_cardinality += 1;
        }
    }

    println!("{}", pass_cardinality);
}
