#[macro_use]
extern crate failure;

use failure::Error;
use std::collections::VecDeque;
use std::cmp::Ordering;

#[derive(Clone)]
struct Connector(u32, u32);

impl Connector {
    fn connects(&self, port: u32) -> Option<u32> {
        if port == self.0 {
            Some(self.1)
        } else if port == self.1 {
            Some(self.0)
        } else {
            None
        }
    }
}

trait FromConnectors {
    fn from_connectors(connectors: &[Connector]) -> Self;
}

fn find_best<T: FromConnectors + Ord>(
    last_port: u32,
    mut available: &mut VecDeque<Connector>,
    mut used: &mut Vec<Connector>,
) -> T {
    let best = T::from_connectors(used);

    (0..available.len())
        .filter_map(|i| {
            if let Some(next_port) = available[i].connects(last_port) {
                used.push(available.remove(i).unwrap());
                let best = find_best(next_port, &mut available, &mut used);
                available.push_front(used.pop().unwrap());
                Some(best)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(best)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct StrongestResult(u32);

impl FromConnectors for StrongestResult {
    fn from_connectors(connectors: &[Connector]) -> Self {
        StrongestResult(
            connectors
                .iter()
                .map(|connector| connector.0 + connector.1)
                .sum(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LongestResult {
    length: usize,
    strength: u32,
}

impl Ord for LongestResult {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.length.cmp(&other.length);
        match ord {
            Ordering::Less | Ordering::Greater => return ord,
            _ => (),
        }
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for LongestResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromConnectors for LongestResult {
    fn from_connectors(connectors: &[Connector]) -> Self {
        LongestResult {
            length: connectors.len(),
            strength: connectors
                .iter()
                .map(|connector| connector.0 + connector.1)
                .sum(),
        }
    }
}

fn strongest_bridge(connectors: &[Connector]) -> u32 {
    find_best::<StrongestResult>(
        0,
        &mut connectors.iter().cloned().collect(),
        &mut Vec::new(),
    ).0
}

fn longest_bridge(connectors: &[Connector]) -> u32 {
    find_best::<LongestResult>(
        0,
        &mut connectors.iter().cloned().collect(),
        &mut Vec::new(),
    ).strength
}

fn parse_connectors(s: &str) -> Result<Vec<Connector>, Error> {
    s.split('\n')
        .map(|line| {
            let ports = line.split('/')
                .map(str::parse::<u32>)
                .take(2)
                .collect::<Result<Vec<_>, _>>()?;
            if ports.len() != 2 {
                bail!("Invalid number of ports");
            }
            Ok(Connector(ports[0], ports[1]))
        })
        .collect()
}

fn main() {
    let input = include_str!("input.txt").trim();
    let connectors = parse_connectors(&input).expect("parse");

    let result = strongest_bridge(&connectors);
    println!("Result 1: {}", result);

    let result = longest_bridge(&connectors);
    println!("Result 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_strongest_test() {
        let input = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";
        let connectors = parse_connectors(&input).expect("parse");

        assert_eq!(strongest_bridge(&connectors), 31);
    }

    #[test]
    fn find_longest_test() {
        let input = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";
        let connectors = parse_connectors(&input).expect("parse");
        assert_eq!(longest_bridge(&connectors), 19);
    }
}
