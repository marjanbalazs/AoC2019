use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Content {
    Asteroid(i32),
    Void,
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Field {
    coord: Point,
    cont: Content,
}

impl Point {
    pub fn vector_product(lhs: Point, rhs: Point) -> i32 {
        lhs.x*rhs.y-rhs.y*lhs.x
    }
    fn rotate_plus_90(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x
        }
    }
    fn rotate_minus_90(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let height = args[2].parse::<usize>().unwrap();
    let width = args[3].parse::<usize>().unwrap();
    let f = File::open(args[1].to_owned())?;
    let mut reader = BufReader::new(f);
    let mut file = String::new();
    let mut line = String::new();
    while let Ok(i) = reader.read_line(&mut line) {
        file.push_str(&line.trim());
        line.clear();
        if i == 0 {
            break;
        }
    }
    let asteroid_field = file
        .chars().enumerate()
        .map(|(idx, elem)| {
            return match elem {
                '.' => Field {
                    coord: Point {
                        x: (idx % width) as i32,
                        y: (idx % height) as i32,
                    },
                    cont: Content::Void,
                },
                '#' => Field {
                    coord: Point {
                        x: (idx % width) as i32,
                        y: (idx % height) as i32,
                    },
                    cont: Content::Asteroid(0),
                },
                _ => panic!(),
            };
        })
        .collect::<Vec<Field>>();

    println!("{:?}", Point { x: 1, y: 2}.rotate_plus_90());
    println!("{:?}", Point { x: 1, y: 2}.rotate_minus_90());
    println!("{}", Point::vector_product(Point { x: 1, y: 2}, Point { x: 1, y: 2}.rotate_minus_90()));
    Ok(())
}
