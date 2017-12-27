#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::str::FromStr;

use failure::Error;
use regex::Regex;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new(r"s(?P<spins>\d+)").unwrap();
            static ref EXCHANGE_RE: Regex = Regex::new(r"x(?P<A>\d+)/(?P<B>\d+)").unwrap();
            static ref PARTNER_RE: Regex = Regex::new(r"p(?P<A>[a-p])/(?P<B>[a-p])").unwrap();
        }

        if let Some(caps) = SPIN_RE.captures(s) {
            Ok(DanceMove::Spin(caps["spins"].parse()?))
        } else if let Some(caps) = EXCHANGE_RE.captures(s) {
            Ok(DanceMove::Exchange(caps["A"].parse()?, caps["B"].parse()?))
        } else if let Some(caps) = PARTNER_RE.captures(s) {
            Ok(DanceMove::Partner(
                caps["A"].chars().nth(0).unwrap(),
                caps["B"].chars().nth(0).unwrap(),
            ))
        } else {
            Err(format_err!("Invalid move: {}", s))
        }
    }
}

struct DanceTeam(VecDeque<char>);

impl ToString for DanceTeam {
    fn to_string(&self) -> String {
        self.0.iter().collect()
    }
}

impl FromIterator<char> for DanceTeam {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        DanceTeam(VecDeque::from_iter(iter))
    }
}

impl DanceTeam {
    fn spin(&mut self, spins: usize) {
        (0..spins).for_each(|_| {
            let back = self.0.pop_back().unwrap();
            self.0.push_front(back);
        });
    }

    fn exchange(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    fn partner(&mut self, a: char, b: char) {
        let a = self.0.iter().position(|c| *c == a).unwrap();
        let b = self.0.iter().position(|c| *c == b).unwrap();
        self.0.swap(a, b);
    }

    fn perform(&mut self, dance_move: &DanceMove) {
        match dance_move {
            &DanceMove::Spin(spins) => self.spin(spins),
            &DanceMove::Exchange(a, b) => self.exchange(a, b),
            &DanceMove::Partner(a, b) => self.partner(a, b),
        }
    }

    fn perform_dance(&mut self, dance_moves: &[DanceMove]) {
        dance_moves
            .iter()
            .for_each(|dance_move| self.perform(dance_move));
    }
}

fn find_pattern(
    dance_team: &mut DanceTeam,
    dance_moves: &[DanceMove],
    max_iters: u64,
) -> Option<(u64, u64)> {
    let mut result_map = HashMap::new();
    for iter in 0..max_iters {
        result_map.insert(dance_team.to_string(), iter);
        dance_team.perform_dance(dance_moves);
        if let Some(&first) = result_map.get(&dance_team.to_string()) {
            return Some((first, iter + 1 - first));
        }
    }
    None
}

fn parse_input(dance: &str) -> Result<Vec<DanceMove>, Error> {
    dance
        .split(',')
        .map(str::parse::<DanceMove>)
        .collect::<Result<Vec<_>, Error>>()
}

fn perform_dance(dance_moves: &[DanceMove]) -> Result<String, Error> {
    let mut dance_team = "abcdefghijklmnop".chars().collect::<DanceTeam>();
    dance_team.perform_dance(dance_moves);
    Ok(dance_team.to_string())
}

fn perform_dance_2(dance_moves: &[DanceMove]) -> Result<String, Error> {
    const DANCE_ITERS: u64 = 1_000_000_000;
    let mut dance_team = "abcdefghijklmnop".chars().collect::<DanceTeam>();
    let (first, reps) =
        find_pattern(&mut dance_team, &dance_moves, DANCE_ITERS).ok_or(format_err!("No pattern"))?;
    let remaining_iters = (DANCE_ITERS - first) % reps;
    (0..remaining_iters).for_each(|_| dance_team.perform_dance(dance_moves));

    Ok(dance_team.to_string())
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    let dance_moves = parse_input(input).expect("parse");

    let order = perform_dance(&dance_moves).expect("parse");
    println!("Result 1: {}", order);

    let order = perform_dance_2(&dance_moves).expect("parse");
    println!("Result 2: {}", order);
}
