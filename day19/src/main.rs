#[macro_use]
extern crate failure;

use failure::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Cell {
    DoNotEnter,
    UpDown,
    LeftRight,
    AllDir,
    Letter(char),
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => Ok(Cell::DoNotEnter),
                        '|' => Ok(Cell::UpDown),
                        '-' => Ok(Cell::LeftRight),
                        '+' => Ok(Cell::AllDir),
                        'A'...'Z' => Ok(Cell::Letter(c)),
                        _ => Err(format_err!("Invalid cell {}.", c)),
                    })
                    .collect::<Result<Vec<_>, Error>>()
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Map { cells: cells })
    }
}

impl Map {
    fn find_start(&self) -> Option<usize> {
        self.cells[0].iter().position(|cell| *cell == Cell::UpDown)
    }

    fn trace(&self) -> Result<(String, usize), Error> {
        let mut letters = Vec::new();
        let mut x = 0;
        let mut y = self.find_start().ok_or(format_err!("No start"))?;
        let mut current_direction = Direction::Down;
        let mut count = 0;

        while x < self.cells.len() && y < self.cells[x].len() {
            count += 1;
            match self.cells[x][y] {
                Cell::UpDown | Cell::LeftRight => match current_direction {
                    Direction::Up => x -= 1,
                    Direction::Down => x += 1,
                    Direction::Left => y -= 1,
                    Direction::Right => y += 1,
                },
                Cell::Letter(l) => {
                    letters.push(l);
                    match current_direction {
                        Direction::Up => x -= 1,
                        Direction::Down => x += 1,
                        Direction::Left => y -= 1,
                        Direction::Right => y += 1,
                    }
                }
                Cell::AllDir => {
                    if current_direction != Direction::Down {
                        match self.cells.get(x - 1).and_then(|c| c.get(y)) {
                            Some(&Cell::UpDown) | Some(&Cell::Letter(_)) => {
                                current_direction = Direction::Up;
                                x -= 1;
                                continue;
                            }
                            _ => (),
                        }
                    }
                    if current_direction != Direction::Up {
                        match self.cells.get(x + 1).and_then(|c| c.get(y)) {
                            Some(&Cell::UpDown) | Some(&Cell::Letter(_)) => {
                                current_direction = Direction::Down;
                                x += 1;
                                continue;
                            }
                            _ => (),
                        }
                    }
                    if current_direction != Direction::Right {
                        match self.cells[x].get(y - 1) {
                            Some(&Cell::LeftRight) | Some(&Cell::Letter(_)) => {
                                current_direction = Direction::Left;
                                y -= 1;
                                continue;
                            }
                            _ => (),
                        }
                    }
                    if current_direction != Direction::Left {
                        match self.cells[x].get(y + 1) {
                            Some(&Cell::LeftRight) | Some(&Cell::Letter(_)) => {
                                current_direction = Direction::Right;
                                y += 1;
                                continue;
                            }
                            _ => (),
                        }
                    }
                    break;
                }
                Cell::DoNotEnter => break,
            };
        }
        Ok((letters.iter().collect(), count - 1))
    }
}

fn main() {
    let input = include_str!("input.txt");

    let map: Map = input.parse().expect("");
    let (result1, result2) = map.trace().expect("");
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}
