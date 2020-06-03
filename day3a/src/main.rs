use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        ((self.x.abs() + self.y.abs())).cmp(&(other.x.abs() + other.y.abs()))
    }
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
            line.clear();
        } else {
            println!("Nothing to do");
            break;
        }
    }

    let mut result: Vec<Point> = Vec::new();

    let wire_a = wires.get(0).unwrap();
    let wire_b = wires.get(1).unwrap();
    
    for point_a in wire_a {
        for point_b in wire_b {
            if point_b == point_a {
                result.push(*point_a);
            }
        }
    }

    result.sort();
    let x = result.get(0).unwrap().x;
    let y = result.get(0).unwrap().y;
    let manhattan_dist = x.abs() + y.abs();
    println!("x: {}, y: {}, Manhattan distance: {}", x ,y ,manhattan_dist);

}
