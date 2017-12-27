extern crate pathfinding;

use pathfinding::Matrix;
use pathfinding::Weights;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Grid {
    matrix: Matrix<Node>,
    position: (usize, usize),
    direction: Direction,
    infections: u32,
    evolved: bool,
}

impl Grid {
    #[allow(dead_code)]
    fn new(size: usize) -> Grid {
        Self::from_matrix(Matrix::new_square(size, Node::Clean))
    }

    fn from_matrix(matrix: Matrix<Node>) -> Grid {
        let size = matrix.rows();
        assert_eq!(size % 2, 1);
        Grid {
            matrix: matrix,
            position: (size / 2, size / 2),
            direction: Direction::Up,
            infections: 0,
            evolved: false,
        }
    }

    fn evolve(mut self) -> Self {
        self.evolved = true;
        self
    }

    fn grow(&mut self) {
        let old_size = self.matrix.rows();
        if self.position.0 < old_size && self.position.1 < old_size {
            return;
        }

        let mut new_matrix = Matrix::new_square(old_size * 3, Node::Clean);
        new_matrix.set_slice(&(old_size, old_size), &self.matrix);
        self.matrix = new_matrix;
        self.position = (
            self.position.0.wrapping_add(old_size),
            self.position.1.wrapping_add(old_size),
        );
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = Matrix::square_from_vec(
            s.chars()
                .filter_map(|c| match c {
                    '#' => Some(Node::Infected),
                    '.' => Some(Node::Clean),
                    _ => None,
                })
                .collect(),
        );
        Ok(Grid::from_matrix(matrix))
    }
}

// TODO: Should use a seperate type for the iterator.
impl Iterator for Grid {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        self.grow();
        match (self.matrix[&self.position], self.evolved) {
            (Node::Clean, false) => {
                self.infections += 1;
                self.matrix[&self.position] = Node::Infected;
                self.direction = match self.direction {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                };
            }
            (Node::Infected, false) => {
                self.matrix[&self.position] = Node::Clean;
                self.direction = match self.direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                };
            }
            (Node::Clean, true) => {
                self.matrix[&self.position] = Node::Weakened;
                self.direction = match self.direction {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                };
            }
            (Node::Infected, true) => {
                self.matrix[&self.position] = Node::Flagged;
                self.direction = match self.direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                };
            }
            (Node::Weakened, _) => {
                self.infections += 1;
                self.matrix[&self.position] = Node::Infected;
            }
            (Node::Flagged, _) => {
                self.matrix[&self.position] = Node::Clean;
                self.direction = match self.direction {
                    Up => Down,
                    Right => Left,
                    Down => Up,
                    Left => Right,
                };
            }
        }
        self.position = match self.direction {
            Up => (self.position.0.wrapping_sub(1), self.position.1),
            Down => (self.position.0.wrapping_add(1), self.position.1),
            Right => (self.position.0, self.position.1.wrapping_add(1)),
            Left => (self.position.0, self.position.1.wrapping_sub(1)),
        };

        Some(self.infections)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    let mut grid: Grid = input.parse().expect("parse");
    let mut grid2 = grid.clone().evolve();

    let result = grid.nth(9_999).expect("iter");
    println!("Result 1: {}", result);

    let result = grid2.nth(9_999_999).expect("iter");
    println!("Result 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let input = ".........\n\
                     .........\n\
                     .........\n\
                     .....#...\n\
                     ...#.....\n\
                     .........\n\
                     .........\n\
                     .........\n\
                     .........";
        let mut grid: Grid = input.parse().unwrap();
        assert_eq!(grid.nth(69).unwrap(), 41);
        assert_eq!(grid.nth(9_929).unwrap(), 5587);
    }

    #[test]
    fn evolve_test() {
        let input = ".........\n\
                     .........\n\
                     .........\n\
                     .....#...\n\
                     ...#.....\n\
                     .........\n\
                     .........\n\
                     .........\n\
                     .........";
        let mut grid: Grid = input.parse::<Grid>().unwrap().evolve();
        assert_eq!(grid.nth(99).unwrap(), 26);
        assert_eq!(grid.nth(9_999_899).unwrap(), 2_511_944);
    }
}
