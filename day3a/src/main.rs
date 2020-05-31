use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

fn wire_route(dirs: Vec<Direction>) -> Vec<Point> {
    let mut wire: Vec<Point> = Vec::new();
    let mut curr_x = 0;
    let mut curr_y = 0;
    for seg in dirs {
        match seg {
            Direction::Up(x) => {
                for _ in 0..x {
                    curr_y += 1;
                    let new_point = Point {
                        x: curr_x,
                        y: curr_y,
                    };
                    wire.push(new_point);
                }
            }
            Direction::Down(x) => {
                for _ in 0..x {
                    curr_y -= 1;
                    let new_point = Point {
                        x: curr_x,
                        y: curr_y,
                    };
                    wire.push(new_point);
                }
            }
            Direction::Left(x) => {
                for _ in 0..x {
                    curr_x += 1;
                    let new_point = Point {
                        x: curr_x,
                        y: curr_y,
                    };
                    wire.push(new_point);
                }
            }
            Direction::Right(x) => {
                for _ in 0..x {
                    curr_x -= 1;
                    let new_point = Point {
                        x: curr_x,
                        y: curr_y,
                    };
                    wire.push(new_point);
                }
            }
        }
    }

    wire
}

fn str_to_direction(s: &str) -> Direction {
    let (dir, num) = s.split_at(1);
    let ret: Direction = match dir {
        "U" => {
            let x = i32::from_str(num).unwrap();
            Direction::Up(x)
        }
        "R" => {
            let x = i32::from_str(num).unwrap();
            Direction::Right(x)
        }
        "D" => {
            let x = i32::from_str(num).unwrap();
            Direction::Down(x)
        }
        "L" => {
            let x = i32::from_str(num).unwrap();
            Direction::Left(x)
        }
        _ => {
            println!("{}", dir);
            panic!("Str_to_Direction")
        }
    };
    return ret;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(args.get(1).unwrap());
    let mut buff_read = BufReader::new(file.unwrap());
    let mut line = String::new();

    let mut wires: Vec<Vec<Point>> = Vec::new();

    while let Ok(i) = buff_read.read_line(&mut line) {
        if i > 0 {
            println!("{}", line);
            let directions: Vec<Direction> = line
                .split(',')
                .map(|l| str_to_direction(l.trim()))
                .collect();

            let wired = wire_route(directions);
            wires.push(wired);
        } else {
            println!("Nothing to do");
            break;
        }
    }
}
