use std::fs;

fn count_integer(slice: &str, int: u32) -> usize {
    let res = slice.chars().fold(0, |acc, x| {
        if x.to_digit(10).unwrap() == int {
            return acc + 1;
        }
        acc
    });
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let x = String::from_utf8(fs::read(args[1].to_owned()).unwrap()).unwrap();
    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();
    let layer_size = width * height;
    let mut slice_border = layer_size;
    let mut min = layer_size;
    let mut final_result = 0;
    let mut res = Vec::new();
    while slice_border != x.len() {
        let slice = &x[slice_border..slice_border+layer_size];
        let zeros = count_integer(slice, 0);
        if min > zeros {
            min = zeros;
            final_result = count_integer(slice, 1) * count_integer(slice, 2)
        }
        res.push(zeros);
        slice_border += layer_size;
    }
    println!("Result: {:?}", final_result);
}
