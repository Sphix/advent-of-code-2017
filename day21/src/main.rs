#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate pathfinding;

use failure::Error;
use pathfinding::Matrix;
use pathfinding::Weights;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pixel {
    On,
    Off,
}

struct Rule(Vec<Matrix<Pixel>>, Matrix<Pixel>);

impl Rule {
    fn new(from: Matrix<Pixel>, to: Matrix<Pixel>) -> Rule {
        Rule(
            vec![
                from.rotated_cw(1),
                from.rotated_cw(2),
                from.rotated_cw(3),
                from.flipped_lr(),
                from.rotated_cw(1).flipped_lr(),
                from.flipped_ud(),
                from.rotated_cw(1).flipped_ud(),
                from, // Has to come last
            ],
            to,
        )
    }
    fn try_apply(&self, grid_slice: &Matrix<Pixel>) -> Option<Matrix<Pixel>> {
        let &Rule(ref from, ref to) = self;
        if from.iter().any(|g| g == grid_slice) {
            Some(to.clone())
        } else {
            None
        }
    }
}

struct Rules {
    two_by_two: Vec<Rule>,
    three_by_three: Vec<Rule>,
}

impl Rules {
    fn apply(&self, grid_slice: &Matrix<Pixel>) -> Result<Matrix<Pixel>, Error> {
        match grid_slice.rows() {
            2 => for rule in self.two_by_two.iter() {
                if let Some(grid_slice) = rule.try_apply(&grid_slice) {
                    return Ok(grid_slice);
                }
            },
            3 => for rule in self.three_by_three.iter() {
                if let Some(grid_slice) = rule.try_apply(&grid_slice) {
                    return Ok(grid_slice);
                }
            },
            _ => bail!("Invalid input."),
        }
        Err(format_err!("No rule matches."))
    }
}

impl FromStr for Rules {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(pixel<&str, Pixel>,
               alt!(do_parse!(tag!("#") >> (Pixel::On)) |
                    do_parse!(tag!(".") >> (Pixel::Off))));

        named!(two <&str, [Pixel; 2]>,
               do_parse!(p0: pixel >> p1: pixel >> ([p0, p1])));

        named!(three<&str, [Pixel; 3]>,
               do_parse!(p0: pixel >> p1: pixel >> p2: pixel >> ([p0, p1, p2])));

        named!(four<&str, [Pixel; 4]>,
               do_parse!(p0: pixel >> p1: pixel >> p2: pixel >> p3: pixel>> ([p0, p1, p2, p3])));

        named!(matrix2<&str, Matrix<Pixel>>,
               do_parse!(l0: two >> tag!("/") >> l1: two >>
                         (Matrix::square_from_vec([l0, l1].concat()))));

        named!(matrix3<&str, Matrix<Pixel>>,
               do_parse!(l0: three >> tag!("/") >> l1: three >> tag!("/") >> l2: three >>
                         (Matrix::square_from_vec([l0, l1, l2].concat()))));

        named!(matrix4<&str, Matrix<Pixel>>,
               do_parse!(l0: four >> tag!("/") >> l1: four >> tag!("/") >>
                         l2: four >> tag!("/") >> l3: four >>
                         (Matrix::square_from_vec([l0, l1, l2, l3].concat()))));

        named!(rule<&str, Rule>,
               alt!(do_parse!(from: matrix2 >> ws!(tag!("=>")) >>
                              to: matrix3 >> (Rule::new(from, to))) |
                    do_parse!(from: matrix3 >> ws!(tag!("=>")) >>
                              to: matrix4 >> (Rule::new(from, to)))));

        let (two_by_two, three_by_three) = s.split('\n')
            .map(rule)
            .map(nom::IResult::to_result)
            .collect::<Result<Vec<Rule>, nom::ErrorKind>>()
            .map_err(|e| format_err!("{}", e))?
            .into_iter()
            .partition(|&Rule(_, ref to)| if to.rows() == 3 { true } else { false });

        Ok(Rules {
            two_by_two: two_by_two,
            three_by_three: three_by_three,
        })
    }
}

struct Grid<'a> {
    matrix: Matrix<Pixel>,
    rules: &'a Rules,
}

impl<'a> Grid<'a> {
    fn new(rules: &'a Rules) -> Self {
        use Pixel::*;
        Grid {
            matrix: Matrix::square_from_vec(vec![Off, On, Off, Off, Off, On, On, On, On]),
            rules: rules,
        }
    }

    fn run_iteration(&mut self) -> Result<(), Error> {
        let rows = self.matrix.rows();
        let (new, old) = if rows % 2 == 0 {
            (3, 2)
        } else if rows % 3 == 0 {
            (4, 3)
        } else {
            bail!("Invalid state.");
        };

        let mut new_grid = Matrix::new_square(rows * new / old, Pixel::Off);
        for i in 0..(rows / old) {
            for j in 0..(rows / old) {
                let grid_slice = self.matrix
                    .slice((i * old)..(i * old + old), (j * old)..(j * old + old));
                new_grid.set_slice(&(i * new, j * new), &self.rules.apply(&grid_slice)?);
            }
        }
        self.matrix = new_grid;
        Ok(())
    }

    fn count_on(&self) -> usize {
        fn count_on(grid_section: &[Pixel]) -> usize {
            grid_section.iter().filter(|p| **p == Pixel::On).count()
        }
        count_on(self.matrix.as_ref())
    }
}

fn on_after_iterations(iterations: usize, rules: &Rules) -> usize {
    let mut grid = Grid::new(rules);
    (0..iterations)
        .map(|_| grid.run_iteration())
        .collect::<Result<Vec<_>, Error>>()
        .expect("iteration.");

    grid.count_on()
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    let rules: Rules = input.parse().expect("parse");

    let result = on_after_iterations(5, &rules);
    println!("Result 1: {}", result);

    let result = on_after_iterations(18, &rules);
    println!("Result 2: {}", result);
}
