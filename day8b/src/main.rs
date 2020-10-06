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

fn add_layers(prev_layer: &str, layer: &str) -> String {
    let zipped = layer.chars().zip(prev_layer.chars());
    let res: String = zipped.map(|pair| {
        if pair.0 == '2' {
            return pair.1;
        }
        else if pair.1 == '2' {
            return pair.0;
        } else {
            return pair.1;
        }
    }).collect();
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let x = String::from_utf8(fs::read(args[1].to_owned()).unwrap()).unwrap();
    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();
    let layer_size = width * height;
    let mut slice_border = layer_size;
    add_layers(&x[0..layer_size], &x[layer_size..2*layer_size]);
    let mut image: String = x[0..layer_size].to_string();
    while slice_border != x.len() {
        let slice = &x[slice_border..slice_border+layer_size];
        image = add_layers(&image, slice);
        slice_border += layer_size;
    }
    let res: String = image.chars().map(|x| {
        if x == '0' {
            return ' ';
        } else {
            return '*';
        }
    }).collect();
    println!("Result: {:?}", &res[0..width]);
    println!("Result: {:?}", &res[width..2*width]);
    println!("Result: {:?}", &res[2*width..3*width]);
    println!("Result: {:?}", &res[3*width..4*width]);
    println!("Result: {:?}", &res[4*width..5*width]);
    println!("Result: {:?}", &res[5*width..6*width]);
}
