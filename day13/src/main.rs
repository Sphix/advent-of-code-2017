#[macro_use]
extern crate failure;
extern crate rayon;

use failure::Error;
use rayon::prelude::*;

type Scanner = (u32, u32);

fn parse_input(input: &str) -> Result<Vec<Scanner>, Error> {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.split(": ");
            let depth = iter.next().ok_or(format_err!("depth"))?.parse::<u32>()?;
            let range = iter.next().ok_or(format_err!("range"))?.parse::<u32>()?;
            Ok((depth, range))
        })
        .collect()
}

fn trip_serverity(firewall: &Vec<Scanner>) -> u32 {
    firewall
        .iter()
        .map(|&(depth, range)| {
            if depth % (range * 2 - 2) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

fn safe_delay(firewall: &Vec<Scanner>) -> u32 {
    (0..std::u32::MAX)
        .into_par_iter()
        .find_first(|delay| {
            firewall
                .iter()
                .all(|&(depth, range)| (depth + delay) % (range * 2 - 2) != 0)
        })
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    let firewall = parse_input(&input).expect("parse");
    let result = trip_serverity(&firewall);
    println!("Result 1: {}", result);

    let result = safe_delay(&firewall);
    println!("Result 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trip_serverity_test() {
        let input = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = parse_input(&input).unwrap();
        assert_eq!(trip_serverity(&firewall), 24);
    }

    #[test]
    fn safe_delay_test() {
        let input = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = parse_input(&input).unwrap();
        assert_eq!(safe_delay(&firewall), 10);
    }
}
