use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
enum Content {
    Asteroid(i32),
    Void,
}
#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub fn vector_product(lhs: &Point, rhs: &Point) -> i32 {
        lhs.x * rhs.y - lhs.y * rhs.x
    }
    pub fn scalar_product(lhs: &Point, rhs: &Point) -> i32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }
    fn rotate_plus_90(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
    fn rotate_minus_90(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
    pub fn is_between(a: &Point, b: &Point, c: &Point) -> bool {
        let vec_prod = Point::vector_product(a, b);
        if vec_prod != 0 {
            return false;
        }
        let scalar_prod = Point::scalar_product(&(*b - *a), &(*c - *a));
        if scalar_prod < 0 {
            return false;
        }
        let squaredlengthba = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);
        if scalar_prod > squaredlengthba {
            return false;
        }
        true
    }
    pub fn is_between_opt(a: &Point, b: &Point, c: &Point) -> bool {
        (a.x - c.x) * (b.y - c.y) == (b.x - c.x) * (a.y - c.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
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
        .chars()
        .enumerate()
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

    Ok(())
}
