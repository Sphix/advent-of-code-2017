#[macro_use]
extern crate failure;
extern crate itertools;

use failure::Error;
use itertools::Itertools;

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn dist(self) -> u32 {
        ((self.x.abs() + self.y.abs() + (self.x + self.y).abs()) / 2) as u32
    }
}

fn find_steps(input: &str) -> Result<(u32, u32), Error> {
    let (max, point) = input
        .split(',')
        .map(|step| match step {
            "n" => Ok(Point { x: 0, y: 1 }),
            "nw" => Ok(Point { x: -1, y: 1 }),
            "sw" => Ok(Point { x: -1, y: 0 }),
            "s" => Ok(Point { x: 0, y: -1 }),
            "se" => Ok(Point { x: 1, y: -1 }),
            "ne" => Ok(Point { x: 1, y: 0 }),
            _ => bail!("Invalid step"),
        })
        .fold_results((0, Point::default()), |(max, current), p| {
            let p = current + p;
            (std::cmp::max(max, p.dist()), p)
        })?;

    Ok((point.dist(), max))
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    let (last, max) = find_steps(input).expect("failed to parse.");
    println!("Result 1: {}", last);
    println!("Result 2: {}", max);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_steps() {
        assert_eq!(find_steps("ne,ne,ne").unwrap(), (3, 3));
        assert_eq!(find_steps("ne,ne,sw,sw").unwrap(), (0, 2));
        assert_eq!(find_steps("ne,ne,s,s").unwrap(), (2, 2));
        assert_eq!(find_steps("se,sw,se,sw,sw").unwrap(), (3, 3));
    }
}
